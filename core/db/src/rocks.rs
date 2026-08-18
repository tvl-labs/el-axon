use std::{error::Error, fs, io, marker::PhantomData, path::Path, sync::Arc};

use rocksdb::ops::{DeleteCF, GetCF, GetColumnFamilys, IterateCF, OpenCF, PutCF, WriteOps};
use rocksdb::{
    ColumnFamily, ColumnFamilyDescriptor, DBIterator, FullOptions, Options, WriteBatch,
    WriteOptions, DB,
};

use common_apm::metrics::storage::on_storage_put_cf;
use common_apm::Instant;
use common_config_parser::types::ConfigRocksDB;
use protocol::codec::{hex_encode, ProtocolCodec};
use protocol::traits::{
    IntoIteratorByRef, StorageAdapter, StorageBatchModify, StorageCategory, StorageIterator,
    StorageSchema,
};
use protocol::{types::Bytes, Display, From, ProtocolError, ProtocolErrorKind, ProtocolResult};

#[derive(Debug)]
pub struct RocksAdapter {
    db: Arc<DB>,
}

impl RocksAdapter {
    /// Create a new RocksDB or load an already existed RocksDB.
    pub fn new<P: AsRef<Path>>(path: P, config: ConfigRocksDB) -> ProtocolResult<Self> {
        Self::open_internal(path, config, true)
    }

    /// Open an already existed RocksDB, or return error if it doesn't exist.
    pub fn open<P: AsRef<Path>>(path: P, config: ConfigRocksDB) -> ProtocolResult<Self> {
        Self::open_internal(path, config, false)
    }

    fn open_internal<P: AsRef<Path>>(
        path: P,
        config: ConfigRocksDB,
        allow_missing: bool,
    ) -> ProtocolResult<Self> {
        if allow_missing && !path.as_ref().is_dir() {
            fs::create_dir_all(&path).map_err(RocksDBError::CreateDB)?;
        }

        let categories = ALL_CATEGORIES.map(map_category);

        let (mut opts, cf_descriptors) = if let Some(ref file) = config.options_file {
            let block_cache_bytes = match config.block_cache_bytes {
                0 => None,
                size => Some(size),
            };

            let mut full_opts = FullOptions::load_from_file(file, block_cache_bytes, false)
                .map_err(RocksDBError::from)?;

            full_opts
                .complete_column_families(&categories, false)
                .map_err(RocksDBError::from)?;
            let FullOptions {
                db_opts,
                cf_descriptors,
            } = full_opts;
            (db_opts, cf_descriptors)
        } else {
            let opts = Options::default();
            let cf_descriptors: Vec<_> = categories
                .into_iter()
                .map(|c| ColumnFamilyDescriptor::new(c, Options::default()))
                .collect();
            (opts, cf_descriptors)
        };

        if allow_missing {
            opts.create_if_missing(true);
        }
        opts.create_missing_column_families(true);
        opts.set_max_open_files(config.max_open_files);

        let db =
            DB::open_cf_descriptors(&opts, path, cf_descriptors).map_err(RocksDBError::from)?;

        Ok(RocksAdapter { db: Arc::new(db) })
    }

    pub fn inner_db(&self) -> Arc<DB> {
        Arc::clone(&self.db)
    }
}

macro_rules! db {
    ($db:expr, $op:ident, $column:expr$ (, $args: expr)*) => {
        $db.$op($column, $($args,)*).map_err(RocksDBError::from)
    };
}

pub struct RocksIterator<'a, S: StorageSchema> {
    inner: DBIterator<'a>,
    pin_s: PhantomData<S>,
}

impl<'a, S: StorageSchema> Iterator for RocksIterator<'a, S> {
    type Item = ProtocolResult<(<S as StorageSchema>::Key, <S as StorageSchema>::Value)>;

    fn next(&mut self) -> Option<Self::Item> {
        let kv_decode = |(k_bytes, v_bytes): (Box<[u8]>, Box<[u8]>)| -> ProtocolResult<_> {
            let key = <_>::decode(k_bytes)?;
            let val = <_>::decode(v_bytes)?;

            Ok((key, val))
        };

        self.inner.next().map(kv_decode)
    }
}

pub struct RocksIntoIterator<'a, S: StorageSchema, P: AsRef<[u8]>> {
    db:     Arc<DB>,
    column: &'a ColumnFamily,
    prefix: &'a P,
    pin_s:  PhantomData<S>,
}

impl<'a, 'b: 'a, S: StorageSchema, P: AsRef<[u8]>> IntoIterator
    for &'b RocksIntoIterator<'a, S, P>
{
    type IntoIter = StorageIterator<'a, S>;
    type Item = ProtocolResult<(<S as StorageSchema>::Key, <S as StorageSchema>::Value)>;

    fn into_iter(self) -> Self::IntoIter {
        let iter: DBIterator<'_> = self
            .db
            .prefix_iterator_cf(self.column, self.prefix.as_ref())
            .unwrap_or_else(|_| panic!("create db {:?} prefix iterator", hex_encode(self.prefix)));

        Box::new(RocksIterator {
            inner: iter,
            pin_s: PhantomData::<S>,
        })
    }
}

impl<'c, S: StorageSchema, P: AsRef<[u8]>> IntoIteratorByRef<S> for RocksIntoIterator<'c, S, P> {
    fn ref_to_iter<'a, 'b: 'a>(&'b self) -> StorageIterator<'a, S> {
        self.into_iter()
    }
}

impl StorageAdapter for RocksAdapter {
    fn insert<S: StorageSchema>(&self, key: S::Key, val: S::Value) -> ProtocolResult<()> {
        let inst = Instant::now();

        let column = get_column::<S>(&self.db)?;
        let key = key.encode()?;
        let val = val.encode()?;
        let size = val.len() as i64;

        db!(self.db, put_cf, column, key, val)?;
        on_storage_put_cf(S::category(), inst.elapsed(), size as f64);

        Ok(())
    }

    fn get<S: StorageSchema>(
        &self,
        key: <S as StorageSchema>::Key,
    ) -> ProtocolResult<Option<<S as StorageSchema>::Value>> {
        let column = get_column::<S>(&self.db)?;
        let key = key.encode()?;

        let opt_bytes = { db!(self.db, get_cf, column, key)? };

        if let Some(bytes) = opt_bytes {
            let val = <_>::decode(bytes)?;

            Ok(Some(val))
        } else {
            Ok(None)
        }
    }

    fn remove<S: StorageSchema>(&self, key: <S as StorageSchema>::Key) -> ProtocolResult<()> {
        let column = get_column::<S>(&self.db)?;
        let key = key.encode()?;

        db!(self.db, delete_cf, column, key)?;

        Ok(())
    }

    fn contains<S: StorageSchema>(&self, key: <S as StorageSchema>::Key) -> ProtocolResult<bool> {
        let column = get_column::<S>(&self.db)?;
        let key = key.encode()?;
        let val = db!(self.db, get_cf, column, key)?;

        Ok(val.is_some())
    }

    fn batch_modify<S: StorageSchema>(
        &self,
        keys: Vec<<S as StorageSchema>::Key>,
        vals: Vec<StorageBatchModify<S>>,
    ) -> ProtocolResult<()> {
        if keys.len() != vals.len() {
            return Err(RocksDBError::BatchLengthMismatch.into());
        }

        let column = get_column::<S>(&self.db)?;
        let mut pairs: Vec<(Bytes, Option<Bytes>)> = Vec::with_capacity(keys.len());

        for (key, value) in keys.into_iter().zip(vals.into_iter()) {
            let key = key.encode()?;

            let value = match value {
                StorageBatchModify::Insert(value) => Some(value.encode()?),
                StorageBatchModify::Remove => None,
            };

            pairs.push((key, value))
        }

        let mut batch = WriteBatch::default();
        let mut insert_size = 0usize;
        let inst = Instant::now();
        for (key, value) in pairs.into_iter() {
            match value {
                Some(value) => {
                    insert_size += value.len();
                    batch.put_cf(column, key, value)
                }
                None => batch.delete_cf(column, key),
            }
            .map_err(RocksDBError::from)?;
        }

        on_storage_put_cf(S::category(), inst.elapsed(), insert_size as f64);

        let mut opt = WriteOptions::default();
        opt.set_sync(true);
        self.db
            .write_opt(&batch, &opt)
            .map_err(RocksDBError::from)?;
        Ok(())
    }

    fn prepare_iter<'a, 'b: 'a, S: StorageSchema + 'static, P: AsRef<[u8]> + 'a>(
        &'b self,
        prefix: &'a P,
    ) -> ProtocolResult<Box<dyn IntoIteratorByRef<S> + 'a>> {
        let column = get_column::<S>(&self.db)?;

        let rocks_iter = RocksIntoIterator {
            db: Arc::clone(&self.db),
            column,
            prefix,
            pin_s: PhantomData::<S>,
        };
        Ok(Box::new(rocks_iter))
    }
}

#[derive(Debug, Display, From)]
pub enum RocksDBError {
    #[display(fmt = "category {} not found", _0)]
    CategoryNotFound(&'static str),

    #[display(fmt = "rocksdb {}", _0)]
    RocksDB(rocksdb::Error),

    #[display(fmt = "parameters do not match")]
    InsertParameter,

    #[display(fmt = "batch length do not match")]
    BatchLengthMismatch,

    #[display(fmt = "Create DB path {}", _0)]
    CreateDB(io::Error),
}

impl Error for RocksDBError {}

impl From<RocksDBError> for ProtocolError {
    fn from(err: RocksDBError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::DB, Box::new(err))
    }
}

const C_VERSION: &str = "c0";
const C_BLOCKS: &str = "c1";
const C_BLOCK_HEADER: &str = "c2";
const C_SIGNED_TRANSACTIONS: &str = "c3";
const C_RECEIPTS: &str = "c4";
const C_WALS: &str = "c5";
const C_HASH_HEIGHT_MAP: &str = "c6";
const C_EVM_CODE_MAP: &str = "c7";
const C_EVM_STATE: &str = "c8";
const C_METADATA_STATE: &str = "c9";
const C_CKB_LIGHT_CLIENT_STATE: &str = "c10";

const ALL_CATEGORIES: [StorageCategory; 11] = [
    StorageCategory::Block,
    StorageCategory::BlockHeader,
    StorageCategory::Receipt,
    StorageCategory::SignedTransaction,
    StorageCategory::Wal,
    StorageCategory::HashHeight,
    StorageCategory::Code,
    StorageCategory::EvmState,
    StorageCategory::MetadataState,
    StorageCategory::CkbLightClientState,
    StorageCategory::Version,
];

pub fn map_category(c: StorageCategory) -> &'static str {
    match c {
        StorageCategory::Block => C_BLOCKS,
        StorageCategory::BlockHeader => C_BLOCK_HEADER,
        StorageCategory::Receipt => C_RECEIPTS,
        StorageCategory::SignedTransaction => C_SIGNED_TRANSACTIONS,
        StorageCategory::Wal => C_WALS,
        StorageCategory::HashHeight => C_HASH_HEIGHT_MAP,
        StorageCategory::Code => C_EVM_CODE_MAP,
        StorageCategory::EvmState => C_EVM_STATE,
        StorageCategory::MetadataState => C_METADATA_STATE,
        StorageCategory::CkbLightClientState => C_CKB_LIGHT_CLIENT_STATE,
        StorageCategory::Version => C_VERSION,
    }
}

pub fn get_column<S: StorageSchema>(db: &DB) -> Result<&ColumnFamily, RocksDBError> {
    let category = map_category(S::category());

    let column = db
        .cf_handle(category)
        .ok_or(RocksDBError::CategoryNotFound(category))?;

    Ok(column)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use rocksdb::ops::{CompactRangeCF, GetPropertyCF};

    // The shipped options file is what deployments actually load, so an invalid
    // option in it would otherwise only show up when a node starts.
    fn shipped_options_file() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../devtools/chain/default.db-options")
    }

    fn filter_block_size(properties: &str) -> u64 {
        properties
            .split(';')
            .filter_map(|property| property.split_once('='))
            .find(|(name, _)| name.trim() == "filter block size")
            .map(|(_, size)| size.trim().parse().expect("filter block size is a number"))
            .unwrap_or_else(|| panic!("no filter block size in table properties: {}", properties))
    }

    // Writes one key into every column family and reports how many bytes of bloom
    // filter each of them ended up with.
    fn filter_block_sizes(config: ConfigRocksDB) -> Vec<(&'static str, u64)> {
        let dir = tempfile::tempdir().unwrap();
        let db = RocksAdapter::new(dir.path(), config).unwrap().inner_db();

        let sizes = ALL_CATEGORIES
            .map(map_category)
            .into_iter()
            .map(|name| {
                let column = db.cf_handle(name).unwrap();

                // Table properties are only reported for SST files, not for
                // memtables, and compacting the range flushes the memtable first.
                db.put_cf(column, b"key", b"value").unwrap();
                db.compact_range_cf(column, None, None);

                let properties = db
                    .property_value_cf(column, "rocksdb.aggregated-table-properties")
                    .unwrap()
                    .unwrap();

                (name, filter_block_size(&properties))
            })
            .collect();

        // Closing the database before removing its directory keeps RocksDB from
        // writing into a path that is already gone.
        drop(db);
        dir.close().unwrap();
        sizes
    }

    #[test]
    fn test_shipped_options_file_enables_bloom_filter_for_every_category() {
        let with_options_file = filter_block_sizes(ConfigRocksDB {
            options_file: Some(shipped_options_file()),
            ..Default::default()
        });

        for (name, size) in with_options_file {
            assert!(size > 0, "column family {} is missing a bloom filter", name);
        }

        // Without the options file no column family has a filter at all, which is
        // what makes the assertion above meaningful.
        let with_defaults = filter_block_sizes(ConfigRocksDB::default());

        for (name, size) in with_defaults {
            assert_eq!(size, 0, "column family {} has an unexpected filter", name);
        }
    }
}

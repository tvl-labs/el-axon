use std::{io, num::NonZeroUsize, sync::Arc};

use lru::LruCache;
use parking_lot::Mutex;
use rocksdb::ops::{GetCF, GetColumnFamilys, PutCF, WriteOps};
use rocksdb::{ColumnFamily, WriteBatch, DB};

use common_apm::metrics::storage::{on_storage_get_state, on_storage_put_state};
use common_apm::Instant;
use protocol::traits::StateStorageCategory;
use protocol::trie;

use core_db::map_category;

// The node cache is sharded so that concurrent trie reads don't serialize on a
// single lock. Keys are node hashes, so the leading byte spreads them evenly.
const CACHE_SHARD_NUM: usize = 16;

macro_rules! db {
    ($db:expr, $op:ident, $column:expr$ (, $args: expr)*) => {
        $db.$op($column, $($args,)*).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("rocksdb error: {:?}", e),
            )
        })?
    };
}

pub struct RocksTrieDB {
    db:       Arc<DB>,
    category: StateStorageCategory,
    cache:    Vec<Mutex<LruCache<Vec<u8>, Vec<u8>>>>,
}

impl trie::DB for RocksTrieDB {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, io::Error> {
        if let Some(val) = self.cache_shard(key).lock().get(key).cloned() {
            return Ok(Some(val));
        }

        let inst = Instant::now();
        let ret = db!(self.db, get_cf, self.get_column(), key);
        on_storage_get_state(inst.elapsed(), 1.0);

        let ret = ret.map(|r| r.to_vec());
        if let Some(val) = &ret {
            self.cache_shard(key)
                .lock()
                .put(key.to_owned(), val.clone());
        }

        Ok(ret)
    }

    fn contains(&self, key: &[u8]) -> Result<bool, io::Error> {
        if self.cache_shard(key).lock().get(key).is_some() {
            Ok(true)
        } else if let Some(val) = db!(self.db, get_cf, self.get_column(), key) {
            self.cache_shard(key)
                .lock()
                .put(key.to_owned(), val.to_vec());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn insert(&self, key: Vec<u8>, value: Vec<u8>) -> Result<(), io::Error> {
        let inst = Instant::now();
        let size = key.len() + value.len();

        db!(self.db, put_cf, self.get_column(), &key, &value);
        self.cache_shard(&key).lock().put(key, value);

        on_storage_put_state(inst.elapsed(), size as f64);
        Ok(())
    }

    fn insert_batch(&self, keys: Vec<Vec<u8>>, values: Vec<Vec<u8>>) -> Result<(), io::Error> {
        if keys.len() != values.len() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "keys and values length not match",
            ));
        }

        let mut total_size = 0;
        let mut batch = WriteBatch::default();
        let column = self.get_column();

        for (key, val) in keys.iter().zip(values.iter()) {
            total_size += key.len();
            total_size += val.len();

            db!(batch, put_cf, column, key, val);
        }

        let inst = Instant::now();
        self.db
            .write(&batch)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("rocksdb error: {:?}", e)))?;
        on_storage_put_state(inst.elapsed(), total_size as f64);

        // Cache the nodes only once they are known to be persisted.
        for (key, val) in keys.into_iter().zip(values.into_iter()) {
            self.cache_shard(&key).lock().put(key, val);
        }

        Ok(())
    }

    fn remove(&self, _key: &[u8]) -> Result<(), io::Error> {
        Ok(())
    }

    fn remove_batch(&self, _keys: &[Vec<u8>]) -> Result<(), io::Error> {
        Ok(())
    }

    fn flush(&self) -> Result<(), io::Error> {
        // Writes go straight to RocksDB and the node cache evicts on insert, so
        // there is nothing left to flush.
        Ok(())
    }
}

impl RocksTrieDB {
    pub fn new_evm(db: Arc<DB>, cache_size: usize) -> Self {
        Self::new(db, StateStorageCategory::EvmState, cache_size)
    }

    pub fn new_metadata(db: Arc<DB>, cache_size: usize) -> Self {
        Self::new(db, StateStorageCategory::MetadataState, cache_size)
    }

    pub fn new_ckb_light_client(db: Arc<DB>, cache_size: usize) -> Self {
        Self::new(db, StateStorageCategory::CkbLightClientState, cache_size)
    }

    fn new(db: Arc<DB>, category: StateStorageCategory, cache_size: usize) -> Self {
        // `cache_size` is the total node count, spread over the shards.
        let shard_size = NonZeroUsize::new(cache_size.div_ceil(CACHE_SHARD_NUM).max(1))
            .expect("shard size is never zero");
        let cache = (0..CACHE_SHARD_NUM)
            .map(|_| Mutex::new(LruCache::new(shard_size)))
            .collect();

        RocksTrieDB {
            db,
            category,
            cache,
        }
    }

    fn cache_shard(&self, key: &[u8]) -> &Mutex<LruCache<Vec<u8>, Vec<u8>>> {
        let idx = key.first().copied().unwrap_or_default() as usize % CACHE_SHARD_NUM;
        &self.cache[idx]
    }

    fn get_column(&self) -> &ColumnFamily {
        let category = map_category(self.category.into());
        self.db
            .cf_handle(category)
            .unwrap_or_else(|| panic!("Column Family {:?} not found", category))
    }

    #[cfg(test)]
    fn cache_len(&self) -> usize {
        self.cache.iter().map(|shard| shard.lock().len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core_db::RocksAdapter;
    use protocol::trie::DB as _;

    const SHARD_CAPACITY: usize = 4;
    const KEYS_PER_SHARD: usize = 64;

    // The leading byte selects the shard, the rest keeps the key unique.
    fn node_key(shard: usize, seq: usize) -> Vec<u8> {
        vec![shard as u8, seq as u8]
    }

    #[test]
    fn test_node_cache_is_bounded_by_cache_size() {
        let dir = tempfile::tempdir().unwrap();
        let inner_db =
            Arc::new(RocksAdapter::new(dir.path(), Default::default()).unwrap()).inner_db();
        let db = RocksTrieDB::new_evm(inner_db, CACHE_SHARD_NUM * SHARD_CAPACITY);

        for seq in 0..KEYS_PER_SHARD {
            for shard in 0..CACHE_SHARD_NUM {
                db.insert(node_key(shard, seq), vec![seq as u8]).unwrap();
            }
        }

        assert_eq!(db.cache_len(), CACHE_SHARD_NUM * SHARD_CAPACITY);

        // Evicted nodes are still served from RocksDB, and reading them back
        // must not push the cache over its capacity either.
        assert_eq!(db.get(&node_key(0, 0)).unwrap(), Some(vec![0]));
        assert_eq!(
            db.get(&node_key(0, KEYS_PER_SHARD - 1)).unwrap(),
            Some(vec![(KEYS_PER_SHARD - 1) as u8])
        );
        assert_eq!(db.cache_len(), CACHE_SHARD_NUM * SHARD_CAPACITY);

        dir.close().unwrap();
    }

    #[test]
    fn test_node_cache_keeps_at_least_one_entry_per_shard() {
        let dir = tempfile::tempdir().unwrap();
        let inner_db =
            Arc::new(RocksAdapter::new(dir.path(), Default::default()).unwrap()).inner_db();
        let db = RocksTrieDB::new_evm(inner_db, 0);

        db.insert_batch(vec![node_key(0, 0), node_key(0, 1)], vec![vec![0], vec![1]])
            .unwrap();

        assert_eq!(db.cache_len(), 1);
        assert_eq!(db.get(&node_key(0, 0)).unwrap(), Some(vec![0]));

        dir.close().unwrap();
    }
}

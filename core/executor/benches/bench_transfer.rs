use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use criterion::{criterion_group, criterion_main, Criterion};

use common_crypto::{
    Crypto, PrivateKey, Secp256k1Recoverable, Secp256k1RecoverablePrivateKey, Signature,
    ToPublicKey, UncompressedPublicKey,
};
use protocol::codec::{hex_decode, ProtocolCodec};
use protocol::traits::Executor;
use protocol::trie::Trie as _;
use protocol::types::{
    public_to_address, Account, Address, Eip1559Transaction, ExecutorContext, Hash, Public,
    SignedTransaction, TxKind, UnsignedTransaction, UnverifiedTransaction, NIL_DATA, RLP_NULL,
    U256, U64,
};

use core_db::RocksAdapter;
use core_executor::{AxonExecutor, AxonExecutorApplyAdapter, MPTTrie, RocksTrieDB};
use core_storage::ImplStorage;

lazy_static::lazy_static! {
    static ref PRIVATE_KEY: Secp256k1RecoverablePrivateKey
        = Secp256k1RecoverablePrivateKey::try_from(hex_decode("95500289866f83502cc1fb894ef5e2b840ca5f867cc9e84ab32fb8872b5dd36c").unwrap().as_ref()).unwrap();
    static ref DISTRIBUTE_ADDRESS: Address = Address::from_str("0x35e70c3f5a794a77efc2ec5ba964bffcc7fd2c0a").unwrap();
}

const DATA_PATH: &str = "../../free-space/rocks/data";

struct BenchAdapter {
    trie_db: Arc<RocksTrieDB>,
    storage: Arc<ImplStorage<RocksAdapter>>,
}

impl BenchAdapter {
    fn new() -> Self {
        let db = RocksAdapter::new(DATA_PATH, Default::default()).unwrap();

        BenchAdapter {
            trie_db: Arc::new(RocksTrieDB::new_evm(db.inner_db(), 1000)),
            storage: Arc::new(ImplStorage::new(Arc::new(db), 100)),
        }
    }

    fn init_mpt(&self) -> Hash {
        let mut mpt = MPTTrie::new(Arc::clone(&self.trie_db));
        let distribute_account = Account {
            nonce:        0u64.into(),
            balance:      U256::from(320000011u64),
            storage_root: RLP_NULL,
            code_hash:    NIL_DATA,
        };

        mpt.insert(
            DISTRIBUTE_ADDRESS.as_slice().to_vec(),
            distribute_account.encode().unwrap().to_vec(),
        )
        .unwrap();

        mpt.commit().unwrap()
    }

    fn init_backend(&self) -> AxonExecutorApplyAdapter<ImplStorage<RocksAdapter>, RocksTrieDB> {
        AxonExecutorApplyAdapter::from_root(
            self.init_mpt(),
            Arc::clone(&self.trie_db),
            Arc::clone(&self.storage),
            ExecutorContext {
                block_number:           U256::from(1),
                block_coinbase:         *DISTRIBUTE_ADDRESS,
                block_timestamp:        U256::from(time_now()),
                chain_id:               U256::from(1),
                origin:                 *DISTRIBUTE_ADDRESS,
                gas_price:              U256::from(85),
                block_gas_limit:        U64::from(100_000_000_000u64),
                block_base_fee_per_gas: Default::default(),
                extra_data:             Default::default(),
            },
        )
        .unwrap()
    }
}

fn time_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn mock_transaction(nonce: u64) -> SignedTransaction {
    let tx = Eip1559Transaction {
        chain_id: 0,
        nonce,
        max_priority_fee_per_gas: 1,
        max_fee_per_gas: 85,
        gas_limit: 1000000,
        value: U256::from(10),
        to: TxKind::Call(*DISTRIBUTE_ADDRESS),
        input: Default::default(),
        access_list: Default::default(),
    };

    let mut utx = UnverifiedTransaction {
        unsigned:  UnsignedTransaction::Eip1559(tx),
        signature: None,
        hash:      Default::default(),
    };

    let raw = utx.signature_hash(true);
    let signature = Secp256k1Recoverable::sign_message(raw.as_slice(), &PRIVATE_KEY.to_bytes())
        .unwrap()
        .to_bytes();
    utx.signature = Some(signature.into());

    let pub_key = Public::from_slice(&PRIVATE_KEY.pub_key().to_uncompressed_bytes()[1..65]);

    SignedTransaction {
        transaction: utx.calc_hash(),
        sender:      public_to_address(&pub_key),
        public:      Some(pub_key),
    }
}

fn mock_txs(number: u64) -> Vec<SignedTransaction> {
    (0..number).map(mock_transaction).collect()
}

fn criterion_100_txs(c: &mut Criterion) {
    let txs = mock_txs(100);
    let mut backend = BenchAdapter::new().init_backend();

    c.bench_function("transfer 100", |b| {
        b.iter(|| AxonExecutor.exec(&mut backend, &txs, &[]))
    });
}

fn criterion_1000_txs(c: &mut Criterion) {
    let txs = mock_txs(1000);
    let mut backend = BenchAdapter::new().init_backend();

    c.bench_function("transfer 1000", |b| {
        b.iter(|| AxonExecutor.exec(&mut backend, &txs, &[]))
    });
}

fn criterion_10000_txs(c: &mut Criterion) {
    let txs = mock_txs(10000);
    let mut backend = BenchAdapter::new().init_backend();

    c.bench_function("transfer 10000", |b| {
        b.iter(|| AxonExecutor.exec(&mut backend, &txs, &[]))
    });
}

criterion_group!(
    benches,
    criterion_100_txs,
    criterion_1000_txs,
    criterion_10000_txs,
);
criterion_main!(benches);

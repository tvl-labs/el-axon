use criterion::{criterion_group, criterion_main, Criterion};
use rand7::{random, rngs::OsRng};

use common_crypto::{
    Crypto, PrivateKey, Secp256k1Recoverable, Secp256k1RecoverablePrivateKey, Signature,
};
use core_consensus::SignedTxsWAL;
use protocol::types::{
    Bytes, BytesMut, Eip1559Transaction, Hash, Hasher, SignatureComponents, SignedTransaction,
    UnsignedTransaction, UnverifiedTransaction,
};

static FULL_TXS_PATH: &str = "./free-space/wal/txs";

fn mock_wal_txs(size: usize) -> Vec<SignedTransaction> {
    (0..size).map(|_| mock_sign_tx()).collect::<Vec<_>>()
}

fn mock_hash() -> Hash {
    Hash::random()
}

pub fn get_random_bytes(len: usize) -> Bytes {
    let vec: Vec<u8> = (0..len).map(|_| random::<u8>()).collect();
    Bytes::from(vec)
}

fn mock_sign_tx() -> SignedTransaction {
    let mut utx = UnverifiedTransaction {
        unsigned:  UnsignedTransaction::Eip1559(Eip1559Transaction {
            chain_id:                 5,
            nonce:                    Default::default(),
            max_priority_fee_per_gas: Default::default(),
            max_fee_per_gas:          Default::default(),
            gas_limit:                Default::default(),
            value:                    Default::default(),
            to:                       Default::default(),
            input:                    Default::default(),
            access_list:              Default::default(),
        }),
        signature: Some(SignatureComponents {
            standard_v: 4,
            r:          Default::default(),
            s:          Default::default(),
        }),
        hash:      mock_hash(),
    }
    .calc_hash();

    let priv_key = Secp256k1RecoverablePrivateKey::generate(&mut OsRng);
    let signature = Secp256k1Recoverable::sign_message(utx.hash.as_slice(), &priv_key.to_bytes())
        .unwrap()
        .to_bytes();
    utx.signature = Some(signature.into());

    SignedTransaction::from_unverified(utx).unwrap()
}

fn criterion_save_wal(c: &mut Criterion) {
    // MacOS M1 Pro, 16GB: time: 933.60µs
    c.bench_function("save wal 1000 txs", |b| {
        let wal = SignedTxsWAL::new(FULL_TXS_PATH);
        let txs = mock_wal_txs(1000);

        let mut buf = BytesMut::new();
        alloy_rlp::encode_list(&txs, &mut buf);
        let txs_hash = Hasher::digest(buf.freeze());

        b.iter(|| {
            wal.save(1u64, txs_hash, txs.clone()).unwrap();
        })
    });
    // MacOS M1 Pro, 16GB: time: 1.80ms
    c.bench_function("save wal 2000 txs", |b| {
        let wal = SignedTxsWAL::new(FULL_TXS_PATH);
        let txs = mock_wal_txs(2000);

        let mut buf = BytesMut::new();
        alloy_rlp::encode_list(&txs, &mut buf);
        let txs_hash = Hasher::digest(buf.freeze());

        b.iter(|| {
            wal.save(1u64, txs_hash, txs.clone()).unwrap();
        })
    });
    // MacOS M1 Pro, 16GB: time: 3.68ms
    c.bench_function("save wal 4000 txs", |b| {
        let wal = SignedTxsWAL::new(FULL_TXS_PATH);
        let txs = mock_wal_txs(4000);

        let mut buf = BytesMut::new();
        alloy_rlp::encode_list(&txs, &mut buf);
        let txs_hash = Hasher::digest(buf.freeze());

        b.iter(|| {
            wal.save(1u64, txs_hash, txs.clone()).unwrap();
        })
    });
    // MacOS M1 Pro, 16GB: time: 7.28ms
    c.bench_function("save wal 8000 txs", |b| {
        let wal = SignedTxsWAL::new(FULL_TXS_PATH);
        let txs = mock_wal_txs(8000);

        let mut buf = BytesMut::new();
        alloy_rlp::encode_list(&txs, &mut buf);
        let txs_hash = Hasher::digest(buf.freeze());

        b.iter(|| {
            wal.save(1u64, txs_hash, txs.clone()).unwrap();
        })
    });
    // MacOS M1 Pro, 16GB: time: 15.33ms
    c.bench_function("save wal 16000 txs", |b| {
        let wal = SignedTxsWAL::new(FULL_TXS_PATH);
        let txs = mock_wal_txs(16000);

        let mut buf = BytesMut::new();
        alloy_rlp::encode_list(&txs, &mut buf);
        let txs_hash = Hasher::digest(buf.freeze());

        b.iter(|| {
            wal.save(1u64, txs_hash, txs.clone()).unwrap();
        })
    });
}

fn criterion_txs_rlp_encode(c: &mut Criterion) {
    // MacOS M1 Pro, 16GB: time: 16.57ms
    c.bench_function("rlp encode 20000 txs", |b| {
        let txs = mock_wal_txs(20000);

        b.iter(|| {
            let mut buf = BytesMut::new();
            alloy_rlp::encode_list(&txs, &mut buf);
            let _ = buf.freeze();
        });
    });
}

criterion_group!(benches, criterion_save_wal, criterion_txs_rlp_encode);
criterion_main!(benches);

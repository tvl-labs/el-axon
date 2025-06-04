mod adapter;
mod storage;

use rand7::{random, rngs::OsRng};

use common_crypto::{
    Crypto, PrivateKey, Secp256k1Recoverable, Secp256k1RecoverablePrivateKey, Signature,
};
use protocol::types::{
    Block, Eip1559Transaction, ExitReason, ExitSucceed, Hash, Hasher, Header, Proof, Receipt,
    SignatureComponents, SignedTransaction, UnverifiedTransaction,
};
use protocol::types::{Bytes, UnsignedTransaction};

fn mock_signed_tx() -> SignedTransaction {
    let mut utx = UnverifiedTransaction {
        unsigned:  UnsignedTransaction::Eip1559(Eip1559Transaction {
            chain_id:                 random::<u64>(),
            nonce:                    Default::default(),
            max_priority_fee_per_gas: Default::default(),
            max_fee_per_gas:          Default::default(),
            value:                    Default::default(),
            gas_limit:                Default::default(),
            to:                       Default::default(),
            input:                    Default::default(),
            access_list:              Default::default(),
        }),
        signature: Some(SignatureComponents {
            standard_v: 4,
            r:          Default::default(),
            s:          Default::default(),
        }),
        hash:      Default::default(),
    }
    .calc_hash();

    let priv_key = Secp256k1RecoverablePrivateKey::generate(&mut OsRng);
    let signature = Secp256k1Recoverable::sign_message(
        utx.signature_hash(true).as_slice(),
        &priv_key.to_bytes(),
    )
    .unwrap()
    .to_bytes();
    utx.signature = Some(signature.into());

    SignedTransaction::from_unverified(utx).unwrap()
}

fn mock_receipt(hash: Hash) -> Receipt {
    Receipt {
        tx_hash:      hash,
        block_number: random::<u64>(),
        block_hash:   Default::default(),
        tx_index:     random::<u32>(),
        state_root:   Default::default(),
        used_gas:     Default::default(),
        logs_bloom:   Default::default(),
        logs:         vec![],
        log_index:    1,
        code_address: None,
        sender:       Default::default(),
        ret:          ExitReason::Succeed(ExitSucceed::Stopped),
        removed:      false,
    }
}

fn mock_block(height: u64, _block_hash: Hash) -> Block {
    let _nonce = Hasher::digest(Bytes::from("XXXX"));
    let header = Header {
        version:                  Default::default(),
        prev_hash:                Default::default(),
        proposer:                 Default::default(),
        state_root:               Default::default(),
        transactions_root:        Default::default(),
        signed_txs_hash:          Default::default(),
        receipts_root:            Default::default(),
        log_bloom:                Default::default(),
        timestamp:                0,
        number:                   height,
        gas_used:                 Default::default(),
        gas_limit:                Default::default(),
        extra_data:               Default::default(),
        base_fee_per_gas:         Default::default(),
        proof:                    Proof::default(),
        call_system_script_count: 1,
        chain_id:                 random::<u64>(),
    };

    Block {
        header,
        tx_hashes: vec![],
    }
}

fn mock_proof(block_hash: Hash) -> Proof {
    Proof {
        number: 0,
        round: 0,
        block_hash,
        signature: Default::default(),
        bitmap: Default::default(),
    }
}

fn get_random_bytes(len: usize) -> Bytes {
    let vec: Vec<u8> = (0..len).map(|_| random::<u8>()).collect();
    Bytes::from(vec)
}

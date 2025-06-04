mod system_script;

use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;

use evm::backend::{MemoryAccount, MemoryVicinity};
use evm::Config;

use protocol::types::{
    AccessList, Address, Bytes, Eip1559Transaction, ExecutorContext, ExitReason, ExitSucceed,
    Public, SignatureComponents, SignedTransaction, TxKind, UnsignedTransaction,
    UnverifiedTransaction, H256, U256, U64,
};
use protocol::{codec::hex_decode, tokio, traits::Executor, trie::MemoryDB};

use core_db::MemoryAdapter;
use core_storage::ImplStorage;

use crate::AxonExecutorApplyAdapter;
use crate::{precompiles::build_precompile_set, AxonExecutor as EvmExecutor, AxonExecutor};

fn exec_adapter() -> AxonExecutorApplyAdapter<ImplStorage<MemoryAdapter>, MemoryDB> {
    let storage = ImplStorage::new(Arc::new(MemoryAdapter::new()), 20);
    let ctx = ExecutorContext {
        block_gas_limit: U64::from(u32::MAX),
        block_base_fee_per_gas: U64::ONE,
        ..Default::default()
    };

    AxonExecutorApplyAdapter::new(Arc::new(MemoryDB::new(false)), Arc::new(storage), ctx).unwrap()
}

fn gen_vicinity() -> MemoryVicinity {
    MemoryVicinity {
        gas_price:              evm_types::U256::zero(),
        origin:                 evm_types::H160::default(),
        block_hashes:           Vec::new(),
        block_number:           Default::default(),
        block_coinbase:         Default::default(),
        block_timestamp:        Default::default(),
        block_difficulty:       Default::default(),
        block_gas_limit:        Default::default(),
        chain_id:               evm_types::U256::one(),
        block_base_fee_per_gas: evm_types::U256::zero(),
    }
}

fn gen_tx(
    sender: evm_types::H160,
    addr: evm_types::H160,
    value: u64,
    data: Vec<u8>,
) -> SignedTransaction {
    SignedTransaction {
        transaction: UnverifiedTransaction {
            unsigned:  UnsignedTransaction::Eip1559(Eip1559Transaction {
                chain_id:                 0,
                nonce:                    0,
                max_priority_fee_per_gas: 0,
                max_fee_per_gas:          0,
                gas_limit:                0x1000000000u64,
                to:                       TxKind::Call(Address::new(addr.0)),
                value:                    U256::from(value),
                input:                    data.into(),
                access_list:              AccessList::default(),
            }),
            signature: Some(SignatureComponents {
                standard_v: 0,
                r:          Bytes::default(),
                s:          Bytes::default(),
            }),
            hash:      H256::default(),
        },
        sender:      Address::new(sender.0),
        public:      Some(Public::default()),
    }
}

#[test]
fn test_ackermann31() {
    let mut state = BTreeMap::new();
    state.insert(
		evm_types::H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
		MemoryAccount {
			nonce: evm_types::U256::one(),
			balance: evm_types::U256::MAX,
			storage: BTreeMap::new(),
			code: hex_decode("60e060020a6000350480632839e92814601e57806361047ff414603457005b602a6004356024356047565b8060005260206000f35b603d6004356099565b8060005260206000f35b600082600014605457605e565b8160010190506093565b81600014606957607b565b60756001840360016047565b90506093565b609060018403608c85600186036047565b6047565b90505b92915050565b6000816000148060a95750816001145b60b05760b7565b81905060cf565b60c1600283036099565b60cb600184036099565b0190505b91905056").unwrap(),
		}
	);
    state.insert(
        evm_types::H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce:   evm_types::U256::one(),
            balance: evm_types::U256::max_value(),
            storage: BTreeMap::new(),
            code:    Vec::new(),
        },
    );

    let mut adapter = exec_adapter();
    let tx = gen_tx(
        evm_types::H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        evm_types::H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        0,
        hex_decode("2839e92800000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000001").unwrap()
    );
    let config = Config::london();
    let precompiles = build_precompile_set();
    let r = EvmExecutor::evm_exec(&mut adapter, &config, &precompiles, &tx);

    assert_eq!(r.exit_reason, ExitReason::Succeed(ExitSucceed::Stopped));
    assert_eq!(r.remain_gas, 68719455392);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_simplestorage() {
    let mut state = BTreeMap::new();
    state.insert(
        evm_types::H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        MemoryAccount {
            nonce:   evm_types::U256::one(),
            balance: evm_types::U256::max_value(),
            storage: BTreeMap::new(),
            code:    Vec::new(),
        },
    );
    let mut adapter = exec_adapter();

    let config = Config::london();
    let precompiles = build_precompile_set();

    // pragma solidity ^0.4.24;
    //
    // contract SimpleStorage {
    //     uint storedData;
    //
    //     function set(uint x) public {
    //         storedData = x;
    //     }
    //
    //     function get() view public returns (uint) {
    //         return storedData;
    //     }
    // }
    //
    // simplestorage_create_code created from above solidity
    let simplestorage_create_code = "608060405234801561001057600080fd5b5060df8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604e5780636d4ce63c146078575b600080fd5b348015605957600080fd5b5060766004803603810190808035906020019092919050505060a0565b005b348015608357600080fd5b50608a60aa565b6040518082815260200191505060405180910390f35b8060008190555050565b600080549050905600a165627a7a7230582099c66a25d59f0aa78f7ebc40748fa1d1fbc335d8d780f284841b30e0365acd960029";
    let mut tx = gen_tx(
        evm_types::H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        evm_types::H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
        0,
        hex_decode(simplestorage_create_code).unwrap(),
    );
    tx.transaction.unsigned.set_action(TxKind::Create);
    let r = EvmExecutor::evm_exec(&mut adapter, &config, &precompiles, &tx);
    assert_eq!(r.exit_reason, ExitReason::Succeed(ExitSucceed::Returned));
    assert!(r.ret.is_empty());
    assert_eq!(r.remain_gas, 68719375495);

    // Thr created contract's address is
    // 0xc15d2ba57d126e6603240e89437efd419ce329d2, you can get the address by
    // `println!("{:?}", backend.state().keys());`

    // let's call SimpleStorage.set(42)
    let tx = gen_tx(
        evm_types::H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        evm_types::H160::from_str("0xc15d2ba57d126e6603240e89437efd419ce329d2").unwrap(),
        0,
        hex_decode("60fe47b1000000000000000000000000000000000000000000000000000000000000002a")
            .unwrap(),
    );
    let r = EvmExecutor::evm_exec(&mut adapter, &config, &precompiles, &tx);
    assert_eq!(r.exit_reason, ExitReason::Succeed(ExitSucceed::Stopped));
    assert!(r.ret.is_empty());
    assert_eq!(r.remain_gas, 68719455532);

    // let's call SimpleStorage.get() by exec
    let tx = gen_tx(
        evm_types::H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
        evm_types::H160::from_str("0xc15d2ba57d126e6603240e89437efd419ce329d2").unwrap(),
        0,
        hex_decode("6d4ce63c").unwrap(),
    );
    let r = EvmExecutor::evm_exec(&mut adapter, &config, &precompiles, &tx);
    assert_eq!(r.exit_reason, ExitReason::Succeed(ExitSucceed::Stopped));
    assert_eq!(r.remain_gas, 68719455672);

    // let's call SimpleStorage.get() by call
    let r = AxonExecutor.call(
        &adapter,
        u64::MAX,
        None,
        Some(Address::from_str("0xc15d2ba57d126e6603240e89437efd419ce329d2").unwrap()),
        U256::default(),
        hex_decode("6d4ce63c").unwrap(),
        false,
    );
    assert_eq!(r.exit_reason, ExitReason::Succeed(ExitSucceed::Stopped));
}

use std::{collections::HashMap, sync::Arc};

use common_crypto::{BlsPrivateKey, PrivateKey};
use protocol::{
    rand,
    tokio::{self, sync::Mutex as AsyncMutex},
    traits::{Context, Synchronization},
    types::{Block, Header, RichBlock},
};

use crate::{
    status::{CurrentStatus, StatusAgent},
    synchronization::OverlordSynchronization,
    tests::MockSyncAdapter,
    util::{time_now, OverlordCrypto},
};

pub fn get_mock_synchronization() -> OverlordSynchronization<MockSyncAdapter> {
    let sync_txs_chunk_size = 50;
    let consensus_adapter = Arc::new(MockSyncAdapter::default());
    let status_agent = StatusAgent::new(CurrentStatus::default());
    let lock = Arc::new(AsyncMutex::new(()));
    let crypto = Arc::new(OverlordCrypto::new(
        BlsPrivateKey::generate(&mut rand::thread_rng()),
        HashMap::new(),
        String::new(),
    ));
    OverlordSynchronization::<_>::new(
        sync_txs_chunk_size,
        consensus_adapter,
        status_agent,
        lock,
        crypto,
    )
}

pub fn get_mock_rick_block() -> RichBlock {
    RichBlock {
        txs:   vec![],
        block: Block {
            tx_hashes: vec![],
            header:    Header {
                version:                  Default::default(),
                prev_hash:                Default::default(),
                proposer:                 Default::default(),
                state_root:               Default::default(),
                transactions_root:        Default::default(),
                signed_txs_hash:          Default::default(),
                receipts_root:            Default::default(),
                log_bloom:                Default::default(),
                timestamp:                time_now(),
                number:                   1,
                gas_used:                 Default::default(),
                gas_limit:                Default::default(),
                extra_data:               Default::default(),
                base_fee_per_gas:         Default::default(),
                proof:                    Default::default(),
                call_system_script_count: 0,
                chain_id:                 0,
            },
        },
    }
}

#[tokio::test]
async fn test_new() {
    let synchronization = get_mock_synchronization();
    tokio::spawn(async move {
        if let Err(e) = synchronization.polling_broadcast().await {
            println!("synchronization: {:?}", e);
        }
    });
    println!("test_new end");
}

#[tokio::test]
async fn test_receive_remote_block() {
    let sync_txs_chunk_size = 50;
    let status_agent = StatusAgent::new(CurrentStatus::default());
    let lock = Arc::new(AsyncMutex::new(()));

    // let network_service = mock_network_service();
    let consensus_adapter = MockSyncAdapter::default();
    let consensus_adapter = Arc::new(consensus_adapter);
    let crypto = Arc::new(OverlordCrypto::new(
        BlsPrivateKey::generate(&mut rand::thread_rng()),
        HashMap::new(),
        String::new(),
    ));
    let synchronization = Arc::new(OverlordSynchronization::<_>::new(
        sync_txs_chunk_size,
        consensus_adapter,
        status_agent.clone(),
        lock,
        crypto,
    ));

    let result = synchronization
        .receive_remote_block(Context::new(), 1)
        .await;
    assert!(result.is_err());
    println!("{:?}", result.err());
}

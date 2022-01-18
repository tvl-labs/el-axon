#![allow(dead_code, unused_variables, clippy::needless_return)]

mod adapter;

pub use adapter::DefaultCrossAdapter;

use std::sync::Arc;

use protocol::traits::{Context, CrossAdapter, CrossClient};
use protocol::types::{BlockNumber, Hash, Header, Log, Proof};
use protocol::{async_trait, ProtocolResult};

pub struct CrossChainImpl<Adapter> {
    adapter: Arc<Adapter>,
}

#[async_trait]
impl<Adapter: CrossAdapter + 'static> CrossClient for CrossChainImpl<Adapter> {
    async fn set_evm_log(
        &self,
        ctx: Context,
        block_number: BlockNumber,
        block_hash: Hash,
        logs: &[Vec<Log>],
    ) -> ProtocolResult<()> {
        Ok(())
    }

    async fn set_checkpoint(
        &self,
        ctx: Context,
        block_header: Header,
        proof: Proof,
    ) -> ProtocolResult<()> {
        Ok(())
    }
}

impl<Adapter: CrossAdapter + 'static> CrossChainImpl<Adapter> {
    pub fn new(adapter: Arc<Adapter>) -> Self {
        CrossChainImpl { adapter }
    }
}
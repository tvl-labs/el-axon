use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::Duration;

use core_executor::{AxonExecutorReadOnlyAdapter, MetadataHandle};
use el_utils::aggregator::Aggregator;
use protocol::traits::{Context, Storage};
use protocol::types::{BlockNumber, Bytes, Hex, H160, H256};
use protocol::{tokio, tokio::time, trie, ProtocolResult};

pub struct Avs<S, DB> {
    storage:              Arc<S>,
    trie:                 Arc<DB>,
    current_block_number: BlockNumber,
    validator_list:       BTreeSet<Bytes>,
    address:              Bytes,
    aggregator:           Aggregator,
}

impl<S, DB> Avs<S, DB>
where
    S: Storage + 'static,
    DB: trie::DB + Send + Sync + 'static,
{
    pub async fn new(
        storage: Arc<S>,
        trie: Arc<DB>,
        address: Bytes,
        eth_url: String,
        ws_url: String,
        signer_pk: Hex,
        signer_addr: H160,
        task_manager_addr: H160,
    ) -> Self {
        let current_block_number = storage
            .get_latest_block_header(Context::new())
            .await
            .unwrap()
            .number;

        let mut ret = Avs {
            storage,
            trie,
            current_block_number,
            validator_list: Default::default(),
            address,
            aggregator: Aggregator::new(eth_url, ws_url, task_manager_addr, signer_pk, signer_addr)
                .await,
        };

        ret.update_validator_list().await.unwrap();
        ret
    }

    pub async fn run(self) {
        tokio::spawn(async move {
            if let Err(e) = self.inner_run().await {
                log::error!("avs run error: {:?}", e);
            }
        });
    }

    async fn inner_run(mut self) -> ProtocolResult<()> {
        loop {
            if let Some(proof) = self.try_update_block_number().await? {
                if self.is_leader().await? {
                    self.aggregator
                        .send_new_task(
                            Bytes::from(proof.as_bytes().to_vec()).into(),
                            67,
                            vec![1u8].into(),
                        )
                        .await;
                }
            } else {
                time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    async fn update_validator_list(&mut self) -> ProtocolResult<()> {
        let ctx = Context::new();
        let latest_header = self.storage.get_latest_block_header(ctx).await?;
        let backend = AxonExecutorReadOnlyAdapter::from_root(
            latest_header.state_root,
            Arc::clone(&self.trie),
            Arc::clone(&self.storage),
            Default::default(),
        )?;
        let metadata_handler = MetadataHandle::new(backend.get_metadata_root());
        let metadata = metadata_handler.get_metadata_by_block_number(latest_header.number)?;
        self.validator_list = metadata
            .verifier_list
            .iter()
            .map(|v| v.pub_key.as_bytes())
            .collect::<BTreeSet<_>>();
        Ok(())
    }

    async fn try_update_block_number(&mut self) -> ProtocolResult<Option<H256>> {
        let ctx = Context::new();
        let latest_header = self.storage.get_latest_block_header(ctx).await.unwrap();
        if latest_header.number > self.current_block_number {
            self.current_block_number = latest_header.number;
            self.update_validator_list().await?;
            return Ok(Some(latest_header.state_root));
        }

        Ok(None)
    }

    async fn is_leader(&self) -> ProtocolResult<bool> {
        let ctx = Context::new();
        let header = self
            .storage
            .get_block_header(ctx, self.current_block_number)
            .await?
            .unwrap();
        let round = header.proof.round;
        let index = (header.number + round) % (self.validator_list.len() as u64);

        Ok(self.validator_list.iter().nth(index as usize).unwrap() == &self.address)
    }
}

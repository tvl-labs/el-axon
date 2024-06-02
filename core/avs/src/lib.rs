pub mod msg;

use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::Duration;

use common_crypto::{BlsPublicKey, BlsSignature, Signature};
use core_executor::{AxonExecutorReadOnlyAdapter, MetadataHandle};
use el_utils::aggregator::Aggregator;
use protocol::traits::{Context, Gossip, Priority, Rpc, Storage};
use protocol::types::{BlockNumber, Bytes, Hex, H160};
use protocol::{tokio, tokio::time, trie, ProtocolResult};

use crate::msg::{AvsSig, RPC_RESP_AVS_SIG};

pub struct Avs<S, DB, N> {
    storage:              Arc<S>,
    trie:                 Arc<DB>,
    network:              Arc<N>,
    current_block_number: u64,
    validator_list:       BTreeSet<Bytes>,
    address:              Bytes,
    aggregator:           Aggregator,
}

impl<S, DB, N> Avs<S, DB, N>
where
    S: Storage + 'static,
    DB: trie::DB + Send + Sync + 'static,
    N: Rpc + Gossip + 'static,
{
    pub async fn new(
        storage: Arc<S>,
        trie: Arc<DB>,
        network: Arc<N>,
        address: Bytes,
        eth_url: String,
        sign_pk: Hex,
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
            network,
            current_block_number,
            validator_list: Default::default(),
            address,
            aggregator: Aggregator::new(eth_url, task_manager_addr, sign_pk),
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
            if self.try_update_block_number().await? {
                if self.is_leader().await? {
                    self.collect_sigs().await?;
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

    async fn try_update_block_number(&mut self) -> ProtocolResult<bool> {
        let ctx = Context::new();
        let latest_header = self.storage.get_latest_block_header(ctx).await.unwrap();
        if latest_header.number > self.current_block_number {
            self.current_block_number = latest_header.number;
            self.update_validator_list().await?;
            return Ok(true);
        }

        Ok(false)
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

    async fn collect_sigs(&mut self) -> ProtocolResult<()> {
        let ctx = Context::new();
        let futs = self
            .validator_list
            .iter()
            .map(|v| {
                self.network.call::<BlockNumber, AvsSig>(
                    ctx.clone(),
                    RPC_RESP_AVS_SIG,
                    Some(v.clone()),
                    self.current_block_number,
                    Priority::High,
                )
            })
            .collect::<Vec<_>>();

        let mut retry_set = self.validator_list.clone();
        retry_set.remove(&self.address);
        let res = futures::future::join_all(futs)
            .await
            .into_iter()
            .filter_map(|r| {
                if r.is_ok() {
                    retry_set.remove(&r.as_ref().unwrap().address);
                }
                r.ok()
            })
            .collect::<Vec<_>>();

        // if res.len() < self.validator_list.len() * 2 / 3 {}

        let aggregated_sig = BlsSignature::combine(
            res.iter()
                .map(|r| {
                    (
                        BlsSignature::try_from(r.sig.as_ref()).unwrap(),
                        BlsPublicKey::try_from(r.address.as_ref()).unwrap(),
                    )
                })
                .collect(),
        )
        .unwrap();

        self.aggregator
            .send_new_task(
                aggregated_sig.to_bytes().into(),
                66,
                Bytes::from(vec![]).into(),
            )
            .await;

        Ok(())
    }
}

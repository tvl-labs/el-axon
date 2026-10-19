use protocol::traits::APIAdapter;
use protocol::types::{
    Account, Block, BlockNumber, Bytes, CallFrame, CkbRelatedInfo, EthAccountProof, HardforkInfo,
    HardforkInfoInner, Hash, Header, Metadata, Proposal, Receipt, SignedTransaction, TxResp, H160,
    H256, U256, U64,
};
use protocol::{async_trait, traits::Context, ProtocolResult};
use std::sync::{
    atomic::{AtomicU64, AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

#[derive(Default)]
pub(super) struct MockAdapter {
    pub head:                   AtomicU64,
    pub reads:                  AtomicUsize,
    pub delay_ms:               AtomicU64,
    pub transactions_per_block: AtomicUsize,
    pub decode_task_delay_ms:   AtomicU64,
    pub decode_tasks_done:      Arc<AtomicUsize>,
    pub logs_per_block:         AtomicUsize,
    pub missing:                AtomicU64,
    pub log_data_bytes:         AtomicUsize,
    pub missing_receipt:        std::sync::atomic::AtomicBool,
    gate:                       std::sync::Mutex<Option<std::sync::mpsc::Receiver<()>>>,
}
impl MockAdapter {
    pub fn pause_next_read(&self) -> std::sync::mpsc::Sender<()> {
        let (tx, rx) = std::sync::mpsc::channel();
        *self.gate.lock().unwrap() = Some(rx);
        tx
    }

    pub fn new(head: u64, delay_ms: u64) -> Arc<Self> {
        Arc::new(Self {
            head: AtomicU64::new(head),
            delay_ms: AtomicU64::new(delay_ms),
            missing: AtomicU64::new(u64::MAX),
            transactions_per_block: AtomicUsize::new(1),
            ..Default::default()
        })
    }
}
#[async_trait]
#[allow(unused_variables)]
impl APIAdapter for MockAdapter {
    async fn get_block_by_number(
        &self,
        _ctx: Context,
        height: Option<u64>,
    ) -> ProtocolResult<Option<Block>> {
        if height.is_some() {
            self.reads.fetch_add(1, Ordering::SeqCst);
            if let Some(rx) = self.gate.lock().unwrap().take() {
                rx.recv_timeout(Duration::from_secs(5)).ok();
            }
            std::thread::sleep(Duration::from_millis(self.delay_ms.load(Ordering::SeqCst)));
        }
        let number = height.unwrap_or_else(|| self.head.load(Ordering::SeqCst));
        if number == self.missing.load(Ordering::SeqCst) {
            return Ok(None);
        }
        Ok(Some(Block {
            header:    Header {
                number,
                ..Default::default()
            },
            tx_hashes: if self.logs_per_block.load(Ordering::SeqCst) > 0 {
                (0..self.transactions_per_block.load(Ordering::SeqCst))
                    .map(|index| H256::from_low_u64_be(index as u64))
                    .collect()
            } else {
                vec![]
            },
        }))
    }

    async fn get_block_by_hash(&self, ctx: Context, _hash: Hash) -> ProtocolResult<Option<Block>> {
        self.get_block_by_number(ctx, Some(1)).await
    }

    async fn get_block_header_by_number(
        &self,
        ctx: Context,
        number: Option<u64>,
    ) -> ProtocolResult<Option<Header>> {
        Ok(self
            .get_block_by_number(ctx, number)
            .await?
            .map(|b| b.header))
    }

    async fn get_receipts_by_hashes(
        &self,
        _ctx: Context,
        height: u64,
        hashes: &[Hash],
    ) -> ProtocolResult<Vec<Option<Receipt>>> {
        // Mirror storage's nested async tasks for batches larger than 1,000
        // receipts.
        let delay = self.decode_task_delay_ms.load(Ordering::SeqCst);
        if hashes.len() > 1_000 && delay > 0 {
            let tasks = hashes
                .chunks(1_000)
                .map(|_| {
                    let done = Arc::clone(&self.decode_tasks_done);
                    protocol::tokio::spawn(async move {
                        std::thread::sleep(Duration::from_millis(delay));
                        done.fetch_add(1, Ordering::SeqCst);
                    })
                })
                .collect::<Vec<_>>();
            for task in tasks {
                task.await.unwrap();
            }
        }
        if self.missing_receipt.load(Ordering::SeqCst) {
            return Ok(vec![None; hashes.len()]);
        }
        Ok(hashes
            .iter()
            .map(|hash| {
                Some(Receipt {
                    tx_hash: *hash,
                    block_number: height,
                    logs: (0..self.logs_per_block.load(Ordering::SeqCst))
                        .map(|_| protocol::types::Log {
                            address: H160::zero(),
                            topics:  vec![],
                            data:    vec![0; self.log_data_bytes.load(Ordering::SeqCst)],
                        })
                        .collect(),
                    ..Default::default()
                })
            })
            .collect())
    }

    async fn insert_signed_txs(
        &self,
        ctx: Context,
        signed_tx: SignedTransaction,
    ) -> ProtocolResult<()> {
        unreachable!("unused test adapter method")
    }

    async fn mempool_contains_tx(&self, ctx: Context, tx_hash: &Hash) -> bool {
        unreachable!("unused test adapter method")
    }

    async fn get_block_number_by_hash(
        &self,
        ctx: Context,
        hash: Hash,
    ) -> ProtocolResult<Option<BlockNumber>> {
        unreachable!("unused test adapter method")
    }

    async fn get_receipt_by_tx_hash(
        &self,
        ctx: Context,
        tx_hash: Hash,
    ) -> ProtocolResult<Option<Receipt>> {
        unreachable!("unused test adapter method")
    }

    async fn get_transaction_by_hash(
        &self,
        ctx: Context,
        tx_hash: Hash,
    ) -> ProtocolResult<Option<SignedTransaction>> {
        unreachable!("unused test adapter method")
    }

    async fn get_transactions_by_hashes(
        &self,
        ctx: Context,
        block_number: u64,
        tx_hashes: &[Hash],
    ) -> ProtocolResult<Vec<Option<SignedTransaction>>> {
        unreachable!("unused test adapter method")
    }

    async fn get_account(
        &self,
        ctx: Context,
        address: H160,
        number: Option<BlockNumber>,
    ) -> ProtocolResult<Account> {
        unreachable!("unused test adapter method")
    }

    async fn get_pending_tx_count(
        &self,
        ctx: Context,
        address: H160,
    ) -> ProtocolResult<(U256, Option<BlockNumber>)> {
        unreachable!("unused test adapter method")
    }

    async fn evm_call(
        &self,
        ctx: Context,
        from: Option<H160>,
        to: Option<H160>,
        gas_price: Option<U64>,
        gas_limit: Option<U64>,
        value: U256,
        data: Vec<u8>,
        estimate: bool,
        state_root: Hash,
        proposal: Proposal,
    ) -> ProtocolResult<TxResp> {
        unreachable!("unused test adapter method")
    }

    async fn evm_trace_call(
        &self,
        ctx: Context,
        stx: &SignedTransaction,
    ) -> ProtocolResult<(TxResp, Option<CallFrame>)> {
        unreachable!("unused test adapter method")
    }

    async fn get_code_by_hash(&self, ctx: Context, hash: &Hash) -> ProtocolResult<Option<Bytes>> {
        unreachable!("unused test adapter method")
    }

    async fn peer_count(&self, ctx: Context) -> ProtocolResult<U256> {
        unreachable!("unused test adapter method")
    }

    async fn get_storage_at(
        &self,
        ctx: Context,
        address: H160,
        position: U256,
        state_root: Hash,
    ) -> ProtocolResult<Bytes> {
        unreachable!("unused test adapter method")
    }

    async fn get_metadata_by_number(
        &self,
        ctx: Context,
        block_number: Option<u64>,
    ) -> ProtocolResult<Metadata> {
        unreachable!("unused test adapter method")
    }

    async fn get_ckb_related_info(&self, ctx: Context) -> ProtocolResult<CkbRelatedInfo> {
        unreachable!("unused test adapter method")
    }

    async fn get_image_cell_root(&self, ctx: Context) -> ProtocolResult<H256> {
        unreachable!("unused test adapter method")
    }

    async fn get_metadata_root(&self, ctx: Context, number: Option<u64>) -> ProtocolResult<H256> {
        unreachable!("unused test adapter method")
    }

    async fn hardfork_info(&self, ctx: Context) -> ProtocolResult<HardforkInfo> {
        unreachable!("unused test adapter method")
    }

    async fn hardfork_proposal(&self, ctx: Context) -> ProtocolResult<Option<HardforkInfoInner>> {
        unreachable!("unused test adapter method")
    }

    async fn get_proof(
        &self,
        _ctx: Context,
        address: H160,
        storage_position: Vec<U256>,
        state_root: Hash,
    ) -> ProtocolResult<EthAccountProof> {
        unreachable!("unused test adapter method")
    }
}

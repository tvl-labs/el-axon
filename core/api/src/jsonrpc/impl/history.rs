use std::future::Future;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use jsonrpsee::core::RpcResult;
use protocol::tokio::{self, runtime::Builder, sync::Semaphore, task::JoinHandle};
use protocol::traits::{APIAdapter, Context};
use protocol::types::{Block, Hash, Hex, H160, H256};

use crate::jsonrpc::{
    error::RpcError,
    web3_types::{BlockId, Web3Filter, Web3Log},
};

const MAX_LOGS: usize = 10_000;

/// One admission budget shared by all historical RPC transports and filter
/// instances.
#[derive(Clone)]
pub(crate) struct HistoryExecutor {
    permits:   Arc<Semaphore>,
    timeout:   Duration,
    max_bytes: usize,
}

impl Default for HistoryExecutor {
    fn default() -> Self {
        Self::new(2, Duration::from_secs(10), 10 * 1024 * 1024).unwrap()
    }
}

impl HistoryExecutor {
    pub(crate) fn new(
        concurrency: usize,
        timeout: Duration,
        max_bytes: usize,
    ) -> Result<Self, String> {
        if concurrency == 0
            || concurrency > Semaphore::MAX_PERMITS
            || timeout.is_zero()
            || Instant::now().checked_add(timeout).is_none()
        {
            return Err(
                "Historical RPC concurrency and timeout must be positive and supported".into(),
            );
        }
        Ok(Self {
            permits: Arc::new(Semaphore::new(concurrency)),
            timeout,
            max_bytes,
        })
    }

    pub(crate) fn start<T, F, Fut>(&self, work: F) -> RpcResult<HistoryJob<T>>
    where
        T: Send + 'static,
        F: FnOnce(QueryBudget) -> Fut + Send + 'static,
        Fut: Future<Output = RpcResult<T>> + Send + 'static,
    {
        // Acquire before spawning: neither the blocking pool nor callers queue
        // for a permit.
        let permit = Arc::clone(&self.permits)
            .try_acquire_owned()
            .map_err(|_| RpcError::HistoryBusy)?;
        let deadline = Instant::now() + self.timeout;
        let cancelled = Arc::new(AtomicBool::new(false));
        let budget = QueryBudget {
            deadline,
            cancelled: Arc::clone(&cancelled),
            removed: None,
            max_bytes: self.max_bytes,
        };
        let worker = tokio::task::spawn_blocking(move || {
            // A timed-out or dropped caller cannot release capacity while
            // storage is still running.
            let _permit = permit;
            budget.check()?;
            // Storage may spawn receipt decode tasks. A per-query runtime keeps
            // those tasks on this blocking thread instead of the consensus
            // runtime.
            let runtime = Builder::new_current_thread()
                .enable_all()
                .max_blocking_threads(1)
                .build()
                .map_err(|err| {
                    RpcError::Internal(format!("Cannot create historical RPC runtime: {err}"))
                })?;
            runtime.block_on(work(budget))
        });
        Ok(HistoryJob {
            worker,
            deadline,
            cancelled,
        })
    }
}

pub(crate) struct HistoryJob<T> {
    worker:    JoinHandle<RpcResult<T>>,
    deadline:  Instant,
    cancelled: Arc<AtomicBool>,
}

impl<T> HistoryJob<T> {
    pub(crate) async fn wait(mut self) -> RpcResult<T> {
        match tokio::time::timeout_at(self.deadline.into(), &mut self.worker).await {
            Ok(result) => result.map_err(|err| {
                RpcError::Internal(format!("Historical RPC worker failed: {err}"))
            })?,
            Err(_) => Err(RpcError::HistoryTimeout.into()),
        }
    }
}

impl<T> Drop for HistoryJob<T> {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
        // This cancels a queued blocking task; a running task cooperatively
        // checks its budget.
        self.worker.abort();
    }
}

pub(crate) struct QueryBudget {
    deadline:  Instant,
    cancelled: Arc<AtomicBool>,
    removed:   Option<Arc<AtomicBool>>,
    max_bytes: usize,
}

impl QueryBudget {
    pub(crate) fn with_removal(mut self, removed: Arc<AtomicBool>) -> Self {
        self.removed = Some(removed);
        self
    }

    pub(crate) fn check(&self) -> RpcResult<()> {
        if self.cancelled.load(Ordering::Acquire)
            || self
                .removed
                .as_ref()
                .map(|flag| flag.load(Ordering::Acquire))
                .unwrap_or(false)
        {
            return Err(RpcError::HistoryCancelled.into());
        }
        if Instant::now() >= self.deadline {
            return Err(RpcError::HistoryTimeout.into());
        }
        Ok(())
    }
}

pub(crate) struct LogQuery {
    pub from:    Option<BlockId>,
    pub to:      Option<BlockId>,
    pub hash:    Option<H256>,
    pub address: Vec<H160>,
    pub topics:  Vec<Option<Vec<Option<Hash>>>>,
}

impl From<Web3Filter> for LogQuery {
    fn from(filter: Web3Filter) -> Self {
        let address: Option<Vec<H160>> = filter.address.into();
        Self {
            from:    filter.from_block,
            to:      filter.to_block,
            hash:    filter.block_hash,
            address: address.unwrap_or_default(),
            topics:  filter
                .topics
                .unwrap_or_default()
                .into_iter()
                .take(4)
                .map(Into::into)
                .collect(),
        }
    }
}

pub(crate) async fn latest_block<A: APIAdapter>(
    adapter: &A,
    budget: &QueryBudget,
) -> RpcResult<Block> {
    budget.check()?;
    let block = adapter
        .get_block_by_number(Context::new(), None)
        .await
        .map_err(|err| RpcError::Internal(err.to_string()))?
        .ok_or(RpcError::CannotGetLatestBlock)?;
    budget.check()?;
    Ok(block)
}

async fn block_at<A: APIAdapter>(
    adapter: &A,
    number: u64,
    budget: &QueryBudget,
) -> RpcResult<Block> {
    budget.check()?;
    let block = adapter
        .get_block_by_number(Context::new(), Some(number))
        .await
        .map_err(|err| RpcError::Internal(err.to_string()))?
        .ok_or(RpcError::CannotFindBlock)?;
    budget.check()?;
    Ok(block)
}

fn number(id: Option<&BlockId>, latest: u64) -> u64 {
    match id {
        Some(BlockId::Num(n)) => n.low_u64(),
        Some(BlockId::Earliest) => 0,
        _ => latest,
    }
}

/// Returns the next cursor only after a complete successful scan.
pub(crate) async fn scan_logs<A: APIAdapter>(
    adapter: &A,
    query: LogQuery,
    max_range: u64,
    polling: bool,
    budget: &QueryBudget,
) -> RpcResult<(Vec<Web3Log>, Option<u64>)> {
    let mut output = LogOutput {
        logs:  Vec::new(),
        bytes: 1024,
    };
    if let Some(hash) = query.hash {
        budget.check()?;
        let block = adapter
            .get_block_by_hash(Context::new(), hash)
            .await
            .map_err(|err| RpcError::Internal(err.to_string()))?
            .ok_or(RpcError::InvalidBlockHash)?;
        output.append(adapter, &block, &query, budget).await?;
        return Ok((output.logs, None));
    }
    let latest = latest_block(adapter, budget).await?;
    let start = number(query.from.as_ref(), latest.header.number);
    let end = number(query.to.as_ref(), latest.header.number);
    if end > latest.header.number {
        return Err(RpcError::InvalidToBlockNumber(end).into());
    }
    if start > latest.header.number {
        return if polling {
            Ok((output.logs, None))
        } else {
            Err(RpcError::InvalidFromBlockNumber(start).into())
        };
    }
    if start > end {
        return Ok((output.logs, None));
    }
    if end - start > max_range {
        return Err(RpcError::InvalidBlockRange(start, end, max_range).into());
    }
    for height in start..=end {
        budget.check()?;
        if height == latest.header.number {
            output.append(adapter, &latest, &query, budget).await?;
        } else {
            let block = block_at(adapter, height, budget).await?;
            output.append(adapter, &block, &query, budget).await?;
        }
    }
    budget.check()?;
    Ok((output.logs, end.checked_add(1)))
}

struct LogOutput {
    logs:  Vec<Web3Log>,
    bytes: usize,
}

impl LogOutput {
    async fn append<A: APIAdapter>(
        &mut self,
        adapter: &A,
        block: &Block,
        query: &LogQuery,
        budget: &QueryBudget,
    ) -> RpcResult<()> {
        budget.check()?;
        let receipts = adapter
            .get_receipts_by_hashes(Context::new(), block.header.number, &block.tx_hashes)
            .await
            .map_err(|err| RpcError::Internal(err.to_string()))?;
        budget.check()?;
        if receipts.len() != block.tx_hashes.len() {
            return Err(RpcError::Internal("Incomplete historical receipts".into()).into());
        }
        for (index, receipt) in receipts.into_iter().enumerate() {
            budget.check()?;
            let receipt =
                receipt.ok_or_else(|| RpcError::Internal("Missing historical receipt".into()))?;
            for (log_index, log) in receipt.logs.iter().enumerate() {
                budget.check()?;
                if !super::web3::log_matches(log, &query.topics, &query.address) {
                    continue;
                }
                if self.logs.len() == MAX_LOGS {
                    return Err(RpcError::HistoryResultTooLarge.into());
                }
                // Conservative JSON upper bound; check before hex encoding or
                // cloning large data.
                let size = 1024usize
                    .saturating_add(log.data.len().saturating_mul(2))
                    .saturating_add(log.topics.len().saturating_mul(70));
                self.bytes = self.bytes.saturating_add(size);
                if self.bytes > budget.max_bytes {
                    return Err(RpcError::HistoryResultTooLarge.into());
                }
                self.logs.push(Web3Log {
                    address:           log.address,
                    topics:            log.topics.clone(),
                    data:              Hex::encode(&log.data),
                    block_hash:        Some(receipt.block_hash),
                    block_number:      Some(receipt.block_number.into()),
                    transaction_hash:  Some(receipt.tx_hash),
                    transaction_index: Some(index.into()),
                    log_index:         Some(log_index.into()),
                    removed:           false,
                });
            }
        }
        budget.check()
    }
}

pub(crate) async fn scan_blocks<A: APIAdapter>(
    adapter: &A,
    start: u64,
    max_range: u64,
    budget: &QueryBudget,
) -> RpcResult<(Vec<H256>, u64)> {
    let latest = latest_block(adapter, budget).await?;
    let end = latest.header.number;
    if start >= end {
        return Ok((Vec::new(), start));
    }
    if end - start > max_range {
        return Err(RpcError::InvalidBlockRange(start, end, max_range).into());
    }
    if (end - start).saturating_mul(70).saturating_add(1024) > budget.max_bytes as u64 {
        return Err(RpcError::HistoryResultTooLarge.into());
    }
    let mut hashes = Vec::new();
    for height in start + 1..end {
        hashes.push(block_at(adapter, height, budget).await?.hash());
    }
    budget.check()?;
    hashes.push(latest.hash());
    budget.check()?;
    Ok((hashes, end))
}

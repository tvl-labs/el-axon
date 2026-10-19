use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use jsonrpsee::core::RpcResult;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use protocol::traits::APIAdapter;
use protocol::types::{Hash, H160, H256, U256, U64};
use protocol::{async_trait, rand::prelude::*, tokio};

use super::history::{latest_block, scan_blocks, scan_logs, HistoryExecutor, LogQuery};
use crate::jsonrpc::web3_types::{BlockId, FilterChanges, RawLoggerFilter};
use crate::jsonrpc::{error::RpcError, Web3FilterServer};

pub(crate) fn filter_module<Adapter: APIAdapter + 'static>(
    adapter: Arc<Adapter>,
    max_range: u64,
    history: HistoryExecutor,
) -> AxonWeb3RpcFilter<Adapter> {
    let filters = Arc::new(Mutex::new(FilterStore {
        entries:    HashMap::new(),
        last_prune: Instant::now(),
    }));
    let weak_filters = Arc::downgrade(&filters);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(20));
        loop {
            interval.tick().await;
            let Some(filters) = weak_filters.upgrade() else {
                break;
            };
            AxonWeb3RpcFilter::<Adapter>::prune(&mut filters.lock());
        }
    });
    AxonWeb3RpcFilter {
        adapter,
        max_range,
        history,
        filters,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LoggerFilter {
    pub from_block: Option<BlockId>,
    pub to_block:   Option<BlockId>,
    pub address:    Option<Vec<H160>>,
    pub topics:     Vec<Option<Vec<Option<Hash>>>>,
}

impl From<RawLoggerFilter> for LoggerFilter {
    fn from(src: RawLoggerFilter) -> Self {
        Self {
            from_block: src.from_block,
            to_block:   src.to_block,
            address:    src.address.into(),
            topics:     src
                .topics
                .unwrap_or_default()
                .into_iter()
                .take(4)
                .map(Into::<Option<Vec<Option<H256>>>>::into)
                .collect(),
        }
    }
}

#[derive(Clone)]
enum FilterKind {
    Logs(LoggerFilter),
    Blocks(u64),
}

struct FilterStore {
    entries:    HashMap<U256, FilterEntry>,
    last_prune: Instant,
}

struct FilterEntry {
    kind:      FilterKind,
    last_poll: Instant,
    state:     Arc<FilterState>,
}

#[derive(Default)]
struct FilterState {
    busy:    AtomicBool,
    removed: Arc<AtomicBool>,
}

// Both the caller and worker hold this lease: timeout cannot admit an
// overlapping poll, and a finished worker cannot admit one before its caller
// commits the new cursor.
struct FilterLease(Arc<FilterState>);
impl Drop for FilterLease {
    fn drop(&mut self) {
        self.0.busy.store(false, Ordering::Release);
    }
}

pub struct AxonWeb3RpcFilter<Adapter> {
    adapter:   Arc<Adapter>,
    max_range: u64,
    history:   HistoryExecutor,
    filters:   Arc<Mutex<FilterStore>>,
}

impl<Adapter: APIAdapter + 'static> AxonWeb3RpcFilter<Adapter> {
    fn prune(filters: &mut FilterStore) {
        if filters.last_prune.elapsed() < Duration::from_secs(20) {
            return;
        }
        filters.last_prune = Instant::now();
        filters.entries.retain(|_, entry| {
            entry.state.busy.load(Ordering::Acquire)
                || entry.last_poll.elapsed() < Duration::from_secs(40)
        });
    }

    fn insert(&self, kind: FilterKind) -> U256 {
        let mut filters = self.filters.lock();
        Self::prune(&mut filters);
        let mut id = random_id();
        while filters.entries.contains_key(&id) {
            id = random_id();
        }
        filters.entries.insert(id, FilterEntry {
            kind,
            last_poll: Instant::now(),
            state: Arc::default(),
        });
        id
    }

    async fn head(&self) -> RpcResult<u64> {
        let adapter = Arc::clone(&self.adapter);
        self.history
            .start(move |budget| async move {
                Ok(latest_block(&*adapter, &budget).await?.header.number)
            })?
            .wait()
            .await
    }

    async fn poll(&self, id: U256) -> RpcResult<FilterChanges> {
        let (kind, lease) = {
            let mut filters = self.filters.lock();
            Self::prune(&mut filters);
            let entry = filters
                .entries
                .get(&id)
                .ok_or(RpcError::CannotFindFilterId(id))?;
            entry
                .state
                .busy
                .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
                .map_err(|_| RpcError::HistoryBusy)?;
            (
                entry.kind.clone(),
                Arc::new(FilterLease(Arc::clone(&entry.state))),
            )
        };
        let worker_lease = Arc::clone(&lease);
        let adapter = Arc::clone(&self.adapter);
        let range = self.max_range;
        let (changes, updated) = self
            .history
            .start(move |budget| async move {
                let _lease = worker_lease;
                let budget = budget.with_removal(Arc::clone(&_lease.0.removed));
                match kind {
                    FilterKind::Logs(mut filter) => {
                        let query = LogQuery {
                            from:    filter.from_block.clone(),
                            to:      filter.to_block.clone(),
                            hash:    None,
                            address: filter.address.clone().unwrap_or_default(),
                            topics:  filter.topics.clone(),
                        };
                        let (logs, next) =
                            scan_logs(&*adapter, query, range, true, &budget).await?;
                        if let Some(next) = next {
                            filter.from_block = Some(BlockId::Num(next.into()));
                        }
                        Ok((FilterChanges::Logs(logs), FilterKind::Logs(filter)))
                    }
                    FilterKind::Blocks(start) => {
                        let (hashes, next) = scan_blocks(&*adapter, start, range, &budget).await?;
                        Ok((FilterChanges::Blocks(hashes), FilterKind::Blocks(next)))
                    }
                }
            })?
            .wait()
            .await?;
        // No await between validating membership and publishing the cursor.
        // Dropped queries leave the old cursor; uninstall cannot
        // resurrect an entry through a late completion.
        let mut filters = self.filters.lock();
        let entry = filters
            .entries
            .get_mut(&id)
            .filter(|entry| Arc::ptr_eq(&entry.state, &lease.0))
            .ok_or(RpcError::CannotFindFilterId(id))?;
        entry.kind = updated;
        entry.last_poll = Instant::now();
        Ok(changes)
    }
}

#[async_trait]
impl<Adapter: APIAdapter + 'static> Web3FilterServer for AxonWeb3RpcFilter<Adapter> {
    async fn new_filter(&self, filter: RawLoggerFilter) -> RpcResult<U256> {
        if let Some(BlockId::Pending) = filter.from_block {
            return Err(RpcError::InvalidFromBlockAndToBlockUnion.into());
        }
        if matches!(
            filter.to_block,
            Some(BlockId::Earliest) | Some(BlockId::Num(U64([0])))
        ) {
            return Err(RpcError::Internal("Invalid to_block".into()).into());
        }
        let mut filter = LoggerFilter::from(filter);
        let head = self.head().await?;
        if !matches!(&filter.from_block, Some(BlockId::Num(n)) if n.low_u64() >= head) {
            filter.from_block = Some(BlockId::Num(head.saturating_add(1).into()));
        }
        Ok(self.insert(FilterKind::Logs(filter)))
    }

    async fn block_filter(&self) -> RpcResult<U256> {
        Ok(self.insert(FilterKind::Blocks(self.head().await?)))
    }

    async fn get_filter_logs(&self, id: U256) -> RpcResult<FilterChanges> {
        self.poll(id).await
    }

    async fn get_filter_changes(&self, id: U256) -> RpcResult<FilterChanges> {
        self.poll(id).await
    }

    async fn uninstall_filter(&self, id: U256) -> RpcResult<bool> {
        let mut filters = self.filters.lock();
        Self::prune(&mut filters);
        if let Some(entry) = filters.entries.remove(&id) {
            entry.state.removed.store(true, Ordering::Release);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn random_id() -> U256 {
    let bytes: [u8; 32] = thread_rng().gen();
    U256::from_big_endian(&bytes)
}

#[cfg(test)]
mod expiry_tests {
    use super::*;
    use crate::jsonrpc::r#impl::test_adapter::MockAdapter;
    use protocol::tokio;

    #[tokio::test]
    async fn idle_filters_are_reclaimed_without_more_filter_requests() {
        let filters = filter_module(MockAdapter::new(1, 0), 25_000, HistoryExecutor::default());
        let id = filters.block_filter().await.unwrap();
        {
            let mut store = filters.filters.lock();
            store.entries.get_mut(&id).unwrap().last_poll =
                Instant::now() - Duration::from_secs(41);
            store.last_prune = Instant::now() - Duration::from_secs(21);
        }
        tokio::time::pause();
        tokio::time::advance(Duration::from_secs(21)).await;
        tokio::task::yield_now().await;
        assert!(
            filters.filters.lock().entries.is_empty(),
            "expired filters survived without another filter RPC"
        );
    }
}

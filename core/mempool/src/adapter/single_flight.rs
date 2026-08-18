use std::{hash::Hash, sync::Arc};

use dashmap::DashMap;

use protocol::tokio::sync::{Mutex, OwnedMutexGuard};

/// Serializes the slow path of a read-through cache per key, so that a burst of
/// requests for the same key only loads it once: the first caller loads while
/// the others wait, and finds the value already cached when it is their turn.
///
/// One entry is kept per key ever seen, so callers are expected to clear the
/// locks whenever the cache they guard is invalidated.
pub struct SingleFlight<K: Eq + Hash> {
    locks: DashMap<K, Arc<Mutex<()>>>,
}

impl<K: Eq + Hash> Default for SingleFlight<K> {
    fn default() -> Self {
        SingleFlight {
            locks: DashMap::new(),
        }
    }
}

impl<K: Eq + Hash> SingleFlight<K> {
    pub async fn acquire(&self, key: K) -> OwnedMutexGuard<()> {
        // The dashmap guard is not `Send` and must not be held across the await.
        let lock = Arc::clone(self.locks.entry(key).or_default().value());
        lock.lock_owned().await
    }

    pub fn clear(&self) {
        self.locks.clear()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.locks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };

    use protocol::tokio::{self, sync::Barrier, time::timeout};

    const CONCURRENCY: usize = 16;

    // Mimics `check_authorization`: read the cache, and on a miss load the value
    // under the single flight lock after re-reading the cache.
    async fn load_through_cache(
        flight: &SingleFlight<u8>,
        cache: &DashMap<u8, u64>,
        loads: &AtomicUsize,
        key: u8,
        entered: &Barrier,
    ) -> u64 {
        if let Some(value) = cache.get(&key) {
            return *value;
        }

        entered.wait().await;

        let _guard = flight.acquire(key).await;
        if let Some(value) = cache.get(&key) {
            return *value;
        }

        // Stands in for the state read, whose await point is what lets the other
        // requests run into the same miss.
        tokio::task::yield_now().await;

        let value = loads.fetch_add(1, Ordering::SeqCst) as u64;
        cache.insert(key, value);
        value
    }

    #[tokio::test]
    async fn test_concurrent_requests_for_one_key_load_once() {
        let flight = Arc::new(SingleFlight::default());
        let cache = Arc::new(DashMap::new());
        let loads = Arc::new(AtomicUsize::new(0));
        // Makes sure every task has missed the cache before any of them loads.
        let entered = Arc::new(Barrier::new(CONCURRENCY));

        let mut tasks = Vec::with_capacity(CONCURRENCY);
        for _ in 0..CONCURRENCY {
            let (flight, cache, loads, entered) = (
                Arc::clone(&flight),
                Arc::clone(&cache),
                Arc::clone(&loads),
                Arc::clone(&entered),
            );

            tasks.push(tokio::spawn(async move {
                load_through_cache(&flight, &cache, &loads, 0, &entered).await
            }));
        }

        for task in tasks {
            assert_eq!(task.await.unwrap(), 0);
        }
        assert_eq!(loads.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_different_keys_load_in_parallel() {
        let flight = Arc::new(SingleFlight::default());
        // Both loads have to be in flight at the same time to get past the
        // barrier, which only works if they don't share a lock.
        let loading = Arc::new(Barrier::new(2));

        let mut tasks = Vec::with_capacity(2);
        for key in 0..2 {
            let (flight, loading) = (Arc::clone(&flight), Arc::clone(&loading));

            tasks.push(tokio::spawn(async move {
                let _guard = flight.acquire(key).await;
                loading.wait().await;
            }));
        }

        for task in tasks {
            timeout(Duration::from_secs(5), task)
                .await
                .expect("locks of different keys must not block each other")
                .unwrap();
        }
    }

    #[tokio::test]
    async fn test_clear_drops_released_locks() {
        let flight = SingleFlight::default();

        for key in 0..2 {
            let _guard = flight.acquire(key).await;
        }
        assert_eq!(flight.len(), 2);

        flight.clear();
        assert_eq!(flight.len(), 0);
    }
}

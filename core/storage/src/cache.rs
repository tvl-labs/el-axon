use std::num::NonZeroUsize;

use lru::LruCache;
use parking_lot::Mutex;

use protocol::types::{Block, Bytes, Hash, Header, Receipt, SignedTransaction};

#[derive(Debug)]
pub struct StorageCache {
    pub blocks:        Mutex<LruCache<u64, Block>>,
    pub block_numbers: Mutex<LruCache<Hash, u64>>,
    pub headers:       Mutex<LruCache<u64, Header>>,
    pub transactions:  Mutex<LruCache<Hash, SignedTransaction>>,
    pub codes:         Mutex<LruCache<Hash, Bytes>>,
    pub receipts:      Mutex<LruCache<Hash, Receipt>>,
}

impl StorageCache {
    pub fn new(size: usize) -> Self {
        let size = NonZeroUsize::new(size.max(1)).unwrap();
        StorageCache {
            blocks:        Mutex::new(LruCache::new(size)),
            block_numbers: Mutex::new(LruCache::new(size)),
            headers:       Mutex::new(LruCache::new(size)),
            transactions:  Mutex::new(LruCache::new(size)),
            codes:         Mutex::new(LruCache::new(size)),
            receipts:      Mutex::new(LruCache::new(size)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StorageCache;

    #[test]
    fn zero_size_uses_minimum_capacity() {
        let cache = StorageCache::new(0);

        assert_eq!(cache.blocks.lock().cap().get(), 1);
        assert_eq!(cache.block_numbers.lock().cap().get(), 1);
        assert_eq!(cache.headers.lock().cap().get(), 1);
        assert_eq!(cache.transactions.lock().cap().get(), 1);
        assert_eq!(cache.codes.lock().cap().get(), 1);
        assert_eq!(cache.receipts.lock().cap().get(), 1);
    }
}

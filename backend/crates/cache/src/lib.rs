//! # Merix Cache Crate
//!
//! **Domain-agnostic** thread-safe wrapper around `dashmap` for any key/value types.
//! Features:
//! - Generic over `K: Hash + Eq + Clone + Debug + Send + Sync` and `V: Clone + Debug + Send + Sync`
//! - Configurable per-instance TTL (lazy expiration on `get` + explicit cleanup)
//! - Full tracing instrumentation

use dashmap::DashMap;
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, trace};

/// Internal value + expiration timestamp.
#[derive(Clone, Debug)]
struct Cached<V> {
    value: V,
    expires_at: Option<Instant>,
}

/// Thread-safe generic cache.
#[derive(Clone, Debug)]
pub struct Cache<K, V>
where
    K: Hash + Eq + Clone + Debug + Send + Sync + 'static,
    V: Clone + Debug + Send + Sync + 'static,
{
    inner: Arc<DashMap<K, Cached<V>>>,
    ttl: Option<Duration>,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone + Debug + Send + Sync + 'static,
    V: Clone + Debug + Send + Sync + 'static,
{
    /// Create a new cache with optional TTL.
    pub fn new(ttl: Option<Duration>) -> Self {
        trace!(ttl = ?ttl, "Cache initialized (generic)");
        Self {
            inner: Arc::new(DashMap::new()),
            ttl,
        }
    }

    fn is_expired(&self, cached: &Cached<V>) -> bool {
        cached
            .expires_at
            .map_or(false, |exp| exp <= Instant::now())
    }

    /// Get a value (auto-removes expired entries).
    pub fn get(&self, key: &K) -> Option<V> {
        if let Some(entry) = self.inner.get(key) {
            if self.is_expired(&entry) {
                drop(entry);
                self.inner.remove(key);
                debug!(?key, "Cache entry expired and removed");
                None
            } else {
                debug!(?key, "Cache hit");
                Some(entry.value().value.clone())
            }
        } else {
            debug!(?key, "Cache miss");
            None
        }
    }

    /// Insert or overwrite a value (resets TTL).
    pub fn insert(&self, key: K, value: V) {
        let expires_at = self.ttl.map(|ttl| Instant::now() + ttl);
        let cached = Cached { value, expires_at };
        self.inner.insert(key.clone(), cached);
        debug!(?key, expires_at = ?expires_at, "Value inserted/updated in cache");
    }

    /// Remove a key.
    pub fn remove(&self, key: &K) -> bool {
        let removed = self.inner.remove(key).is_some();
        if removed {
            debug!(?key, "Value removed from cache");
        }
        removed
    }

    /// Check if a key exists (and is not expired).
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Current number of live entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Clear everything.
    pub fn clear(&self) {
        self.inner.clear();
        debug!("Cache cleared");
    }

    /// Explicitly remove all expired entries.
    pub fn cleanup_expired(&self) {
        let now = Instant::now();
        let mut count = 0usize;
        self.inner.retain(|_, cached| {
            if cached.expires_at.map_or(false, |exp| exp <= now) {
                count += 1;
                false
            } else {
                true
            }
        });
        if count > 0 {
            debug!(expired_removed = count, "Cache cleanup completed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_basic_operations() {
        let cache: Cache<String, String> = Cache::new(Some(Duration::from_millis(200)));

        cache.insert("key1".to_string(), "value1".to_string());
        assert!(cache.contains_key(&"key1".to_string()));
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"key1".to_string()).as_deref(), Some("value1"));

        thread::sleep(Duration::from_millis(250));
        assert!(cache.get(&"key1".to_string()).is_none());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_vec_as_value() {
        let cache: Cache<String, Vec<String>> = Cache::new(Some(Duration::from_millis(150)));

        let mut history = vec!["msg1".to_string()];
        cache.insert("conv-1".to_string(), history.clone());
        history.push("msg2".to_string());
        cache.insert("conv-1".to_string(), history);

        let retrieved = cache.get(&"conv-1".to_string()).unwrap();
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[1], "msg2");

        thread::sleep(Duration::from_millis(200));
        assert!(cache.get(&"conv-1".to_string()).is_none());
    }

    #[test]
    fn test_cleanup_expired() {
        let cache: Cache<String, i32> = Cache::new(Some(Duration::from_millis(50)));
        cache.insert("expiring".to_string(), 42);

        let permanent: Cache<String, String> = Cache::new(None);
        permanent.insert("forever".to_string(), "stays".to_string());

        thread::sleep(Duration::from_millis(100));

        cache.cleanup_expired();
        permanent.cleanup_expired();

        assert_eq!(cache.len(), 0);
        assert_eq!(permanent.len(), 1);
    }
}

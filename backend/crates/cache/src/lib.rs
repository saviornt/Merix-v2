//! # Merix Cache Crate
//!
<<<<<<< HEAD
//! **Domain-agnostic** thread-safe wrapper around `dashmap` for any key/value types.
//! Features:
//! - Generic over `K: Hash + Eq + Clone + Debug + Send + Sync` and `V: Clone + Debug + Send + Sync`
//! - Configurable per-instance TTL (lazy expiration on `get` + explicit cleanup)
//! - Full tracing instrumentation
=======
//! Provides an API wrapper around `dashmap` for concurrent, in-memory caching of sessions
//! and conversation contexts. This crate abstracts the underlying `DashMap` to provide
//! a clean, type-safe, and idiomatic Rust API for the Merix backend.
>>>>>>> 636d75425dcb2e61e1246350c5ec0e692ec2b0af

use dashmap::DashMap;
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, trace};

<<<<<<< HEAD
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
=======
/// A thread-safe, concurrent cache for user sessions.
///
/// Wraps an `Arc<DashMap>` to provide convenient methods for session management
/// without exposing the underlying concurrent map directly.
#[derive(Clone, Debug)]
pub struct SessionCache(Arc<DashMap<String, merix_core::Session>>);

impl SessionCache {
    /// Creates a new, empty session cache.
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }

    /// Retrieves a session by its ID, returning a clone of the session if found.
    pub fn get(&self, id: &str) -> Option<merix_core::Session> {
        self.0.get(id).map(|entry| entry.value().clone())
    }

    /// Inserts or updates a session in the cache.
    pub fn insert(&self, id: String, session: merix_core::Session) {
        self.0.insert(id, session);
    }

    /// Removes a session by ID and returns it if it existed.
    pub fn remove(&self, id: &str) -> Option<merix_core::Session> {
        self.0.remove(id).map(|(_, session)| session)
    }

    /// Checks if a session exists for the given ID.
    pub fn contains_key(&self, id: &str) -> bool {
        self.0.contains_key(id)
    }

    /// Returns the number of sessions currently cached.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the cache contains no sessions.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears all sessions from the cache.
    pub fn clear(&self) {
        self.0.clear();
    }
}

/// A thread-safe, concurrent cache for conversation contexts (lists of messages).
///
/// Wraps an `Arc<DashMap>` for storing per-context (e.g., per-session or per-user)
/// message histories.
#[derive(Clone, Debug)]
pub struct ContextCache(Arc<DashMap<String, Vec<merix_core::Message>>>);

impl ContextCache {
    /// Creates a new, empty context cache.
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }

    /// Retrieves the list of messages for a given context key, cloning the vector if found.
    pub fn get(&self, key: &str) -> Option<Vec<merix_core::Message>> {
        self.0.get(key).map(|entry| entry.value().clone())
    }

    /// Inserts or updates the message list for a context key.
    pub fn insert(&self, key: String, messages: Vec<merix_core::Message>) {
        self.0.insert(key, messages);
    }

    /// Appends a single message to an existing context (or creates a new one if none exists).
    /// This is a convenience method for building conversation histories.
    pub fn push_message(&self, key: String, message: merix_core::Message) {
        let mut entry = self.0.entry(key).or_default();
        entry.push(message);
    }

    /// Removes the context for a given key and returns the previous message list if any.
    pub fn remove(&self, key: &str) -> Option<Vec<merix_core::Message>> {
        self.0.remove(key).map(|(_, messages)| messages)
    }

    /// Checks if a context exists for the given key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Returns the number of contexts currently cached.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the cache contains no contexts.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears all contexts from the cache.
    pub fn clear(&self) {
        self.0.clear();
    }
}

/// Initializes both session and context caches.
///
/// Returns a tuple of the two cache instances, ready for use in the application.
pub fn init_caches() -> (SessionCache, ContextCache) {
    (SessionCache::new(), ContextCache::new())
}

// Note: The previous `placeholder` function has been replaced with the full API wrapper.
>>>>>>> 636d75425dcb2e61e1246350c5ec0e692ec2b0af

//! # Merix Cache Crate
//!
//! Provides an API wrapper around `dashmap` for concurrent, in-memory caching of sessions
//! and conversation contexts. This crate abstracts the underlying `DashMap` to provide
//! a clean, type-safe, and idiomatic Rust API for the Merix backend.

use dashmap::DashMap;
use std::sync::Arc;

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

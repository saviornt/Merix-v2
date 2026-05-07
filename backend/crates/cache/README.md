# merix-cache

**Domain-agnostic concurrent in-memory cache layer** — a thin, type-safe wrapper around **`dashmap`** with built-in configurable TTL, lazy expiration, and full tracing support.

This crate provides a single generic `Cache<K, V>` type that can be used for **anything** (sessions, conversation histories, embeddings, rate-limit counters, etc.) while remaining completely decoupled from the rest of the Merix domain.

All operations are thread-safe, zero-copy where possible, and production-ready.

## Table of Contents

* [Quick Start](#quick-start)
* [Initialization](#initialization)
* [Core API](#core-api)
* [TTL & Expiration](#ttl--expiration)
* [Tracing & Observability](#tracing--observability)
* [Best Practices](#best-practices)

## Quick Start

```toml
# In any other crate (e.g. backend/tauri, merix-memory, etc.)
[dependencies]
merix-cache = { workspace = true }
```

```rust
use merix_cache::Cache;
use std::time::Duration;

// Create caches with optional TTL
let session_cache: Cache<String, Session> = Cache::new(Some(Duration::from_secs(3600)));
let context_cache: Cache<String, Vec<Message>> = Cache::new(None); // no expiration
```

## Initialization

```rust
// Usually called once at application startup (Tauri or main binary)
use merix_cache::Cache;
use std::time::Duration;

let session_cache: Cache<String, Session> = Cache::new(Some(Duration::from_secs(3600)));
let context_cache: Cache<String, Vec<Message>> = Cache::new(Some(Duration::from_secs(1800)));

// You can also create as many independent caches as you need
let rate_limit_cache: Cache<String, u32> = Cache::new(Some(Duration::from_secs(60)));
```

**Global singleton pattern** (optional, for Tauri commands):

```rust
// In your main Tauri setup (once)
merix_cache::init_global_caches(...) // if you added it later

// Then anywhere:
let sessions = merix_cache::global_session_cache();
```

## Core API

| Method                  | Description                                      | Example |
|-------------------------|--------------------------------------------------|---------|
| `new(ttl)`              | Create cache with optional TTL                   | `Cache::new(Some(Duration::from_secs(3600)))` |
| `insert(key, value)`    | Insert / overwrite (resets TTL)                  | `cache.insert("user-123".into(), session)` |
| `get(key)`              | Retrieve value (auto-removes expired entries)    | `cache.get(&key)` → `Option<V>` |
| `contains_key(key)`     | Check existence (non-expired)                    | `if cache.contains_key(&key) { ... }` |
| `remove(key)`           | Remove a single entry                            | `cache.remove(&key)` |
| `len()`                 | Number of live entries                           | `cache.len()` |
| `clear()`               | Remove **all** entries                           | `cache.clear()` |
| `cleanup_expired()`     | Manually purge expired entries                   | `cache.cleanup_expired()` |

### Example: Session + Context Caches

```rust
use merix_cache::Cache;
use std::time::Duration;

// Session cache (expires after 1 hour)
let sessions: Cache<String, Session> = Cache::new(Some(Duration::from_secs(3600)));

// Context cache (no TTL — lives until manually cleared)
let contexts: Cache<String, Vec<Message>> = Cache::new(None);

// Push message to conversation history
let msg = Message { role: "user".into(), content: "Hello".into() };
contexts.insert("conv-abc".into(), vec![msg.clone()]);   // or use push_message helper if you add one later
```

## TTL & Expiration

* TTL is **lazy**: expiration is only checked on `get()` / `contains_key()`.
* Every `insert()` resets the TTL for that key.
* `cleanup_expired()` is available for background tasks or periodic maintenance.
* `None` = never expires (permanent cache).

## Tracing & Observability

The crate emits structured tracing events at:

* `trace!` – cache initialization
* `debug!` – every `get` / `insert` / `remove`
* `debug!` – expiration events

Configure your root `tracing` subscriber as usual (already set up in the workspace).

## Best Practices

1. **Create caches once** at application startup and clone them freely (`Cache` is `Clone + Send + Sync`).
2. **Keep the crate domain-agnostic** — never add `merix-core` types inside `merix-cache`.
3. **Use separate cache instances** for different concerns (sessions, contexts, rate-limits, etc.).
4. **Call `cleanup_expired()`** from a background task if you have very long-lived caches with short TTLs.
5. **Always use the workspace dependency** (`merix-cache = { workspace = true }`).
6. **Test with realistic TTL values** — the integration tests already cover concurrent access and expiration.

---

**Repository**: [https://github.com/saviornt/Merix-v2](https://github.com/saviornt/Merix-v2)  
**Version**: 0.1.0 (matches workspace)

This README serves as the living API reference for the `merix-cache` crate.

Happy caching! 🚀

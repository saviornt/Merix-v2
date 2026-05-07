# merix-db

**Multi-model data layer for Merix** — a thin, domain-agnostic abstraction over **SurrealDB v3** (embedded RocksDB / in-memory).

This crate provides a clean, type-safe Rust API for all of SurrealDB’s data models:

- **Document** – CRUD + batch operations  
- **Vector** – embeddings + similarity search  
- **Graph** – relations, edges, traversals  
- **Full-text** – lexical search with BM25 scoring  
- **Geospatial** – radius + distance queries  
- **Time-series** – range + latest-N queries  

All functions are `async`, production-ready, and return `Result<T, merix_core::MerixError>`.

## Table of Contents

- [Quick Start](#quick-start)
- [Initialization](#initialization)
- [Schema Management](#schema-management)
- [Document Operations](#document-operations)
- [Vector Search](#vector-search)
- [Graph Operations](#graph-operations)
- [Full-Text Search](#full-text-search)
- [Geospatial Queries](#geospatial-queries)
- [Time-Series Queries](#time-series-queries)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)

## Quick Start

```toml
# In any other crate (e.g. merix-memory, merix-orchestration, backend/tauri)
[dependencies]
merix-db = { workspace = true }
```

```rust
use merix_db::{Db, init, document, vector_search, graph, /* ... */};
use merix_core::MerixError;

#[tokio::main]
async fn main() -> Result<(), MerixError> {
    let db: Db = init().await?;

    // Your schemas (once)
    merix_db::apply_schemas(&db, &[
        // your DEFINE INDEX / DEFINE TABLE statements
    ]).await?;

    Ok(())
}
```

## Named Connections (MerixDbPool)

Merix uses three named embedded connections with distinct storage characteristics:

```rust
let pool = merix_db::MerixDbPool::init().await?;

// Use the appropriate connection for each workload
let persistent = pool.standard();   // RocksDB  → durable main storage
let temporal  = pool.temporal();    // SurrealKV → versioned / temporal AI memory
let hot       = pool.ephemeral();   // Mem       → fast in-memory (no persistence)
```

### Directory Structure on Disk

```text
Merix/
├── databases/
├    ├── standard_db/      ← RocksDB (main persistent storage)
├    └── temporal_db/      ← SurrealKV (temporal/versioned memory)
└── models/                ← GGUF models
```

Ephemeral connection is always pure in-memory and creates no folder.

## Initialization

```rust
// One-call initialization (handles connection + health check)
pub async fn init() -> Result<Db, MerixError> {
    merix_db::init().await
}
```

`Db` is `Arc<Surreal<Any>>` — thread-safe and reuse-ready for heavy concurrent workloads.

## Schema Management

All indexes and analyzers are defined **once** (idempotent):

```rust
merix_db::apply_schemas(&db, &[
    "DEFINE INDEX idx_embedding ON vectors FIELDS embedding HNSW DIMENSION 1536 DIST COSINE",
    // ... other indexes
]).await?;
```

Helper functions exist for convenience:

- `merix_db::define_full_text_index(...)`
- `merix_db::define_geospatial_index(...)`
- `merix_db::define_time_series_index(...)`

## Document Operations

```rust
use merix_db::document;

// Create
let id = document::insert(&db, "users", user_data).await?;

// Batch
let ids = document::insert_many(&db, "events", vec![e1, e2, e3]).await?;

// Read
let all: Vec<User> = document::find_all(&db, "users").await?;
let one: Option<User> = document::find_by_id(&db, "user:123").await?;

// Update / Delete
document::update(&db, "user:123", updates).await?;
document::delete(&db, "user:123").await?;
```

## Vector Search

```rust
use merix_db::vector_search;

// Upsert vectors (pre-computed embeddings from merix-inference)
vector_search::upsert(&db, "vectors", items, None).await?;           // auto IDs
vector_search::upsert(&db, "vectors", items, Some(ids)).await?;      // explicit IDs

// Search
let results: Vec<VectorSearchResult<MyDoc>> = vector_search::search(
    &db,
    "vectors",
    VectorQuery { embedding: vec![...], limit: 10 },
    None,                     // optional filter_id
).await?;
```

**Note**: Embeddings must be generated **before** calling `upsert` (see `merix-inference` crate).

## Graph Operations

```rust
use merix_db::graph;

// Create edge
let edge_id = graph::create_edge(&db, "user:1", "follows", "user:2", Some(metadata)).await?;

// Traverse
let friends: Vec<User> = graph::traverse(
    &db,
    "user:1",
    GraphDirection::Out,
    "follows",
    Some(2),   // depth
    Some(50),  // limit
).await?;
```

## Full-Text Search

```rust
use merix_db::full_text_search;

// Search
let docs: Vec<MyDoc> = full_text_search::search_text(
    &db,
    "documents",
    "content",          // field to search
    "rust ai runtime",
    20,
).await?;
```

Define the index once:

```rust
full_text_search::define_full_text_index(&db, "documents", "content", "english_analyzer", true).await?;
```

## Geospatial Queries

```rust
use merix_db::geospatial;

// Find places within radius (meters)
let nearby: Vec<Place> = geospatial::nearby(
    &db,
    "places",
    "location",               // geometry::point field
    (-112.0740, 33.4484),     // (lon, lat)
    5000.0,                   // 5 km
    20,
).await?;
```

Define index once:

```rust
geospatial::define_geospatial_index(&db, "places", "location").await?;
```

## Time-Series Queries

```rust
use merix_db::time_series;
use chrono::{DateTime, Utc};

// Time window
let window: Vec<Event> = time_series::range(
    &db,
    "sensor_data",
    "timestamp",
    start,
    end,
    Some(100),
).await?;

// Most recent N records
let latest: Vec<Event> = time_series::latest(&db, "sensor_data", "timestamp", 50).await?;
```

Define index once:

```rust
time_series::define_time_series_index(&db, "sensor_data", "timestamp").await?;
```

## Error Handling

All functions return `Result<T, merix_core::MerixError>`.  
The error variant `MerixError::Db(String)` wraps any SurrealDB error with context.

## Best Practices

1. **Call `init()` once** at application startup.
2. **Define indexes once** (use `IF NOT EXISTS` or the helper functions).
3. **Pre-compute embeddings** before calling `vector_search::upsert`.
4. **Use the workspace dependency** (`merix-db = { workspace = true }`).
5. **Keep the db crate domain-agnostic** — all business logic lives in `merix-memory`, `merix-agent`, etc.

---

**Repository**: [https://github.com/saviornt/Merix-v2](https://github.com/saviornt/Merix-v2)  
**Version**: 0.0.1 (matches workspace)

This README serves as the living API reference for the `merix-db` crate.  
Happy building! 🚀

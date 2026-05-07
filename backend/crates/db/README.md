# merix-db

**Multi-model data layer for Merix** — a clean, domain-agnostic abstraction over SurrealDB v3 (embedded only).

This crate provides type-safe Rust APIs for all of SurrealDB’s data models:

- **Document** – CRUD + batch operations  
- **Vector** – embeddings + similarity search  
- **Graph** – relations, edges, traversals  
- **Full-text** – lexical search with BM25 scoring  
- **Geospatial** – radius + distance queries  
- **Time-series** – range + latest-N queries  

All functions are `async` and return `Result<T, merix_core::MerixError>`.

## Directory Structure

```text
Merix/
├── databases/
│   ├── standard_db/      ← RocksDB (main persistent storage)
│   └── temporal_db/      ← SurrealKV (temporal/versioned AI memory)
└── models/               ← GGUF models
```

## Named Connections (`MerixDbPool`)

```rust
use merix_db::MerixDbPool;

let pool = MerixDbPool::init().await?;

// Use the appropriate connection for each workload
let persistent = pool.standard();   // RocksDB  → durable main storage
let temporal  = pool.temporal();    // SurrealKV → versioned / temporal AI memory
let hot       = pool.ephemeral();   // Mem       → fast in-memory (no persistence)
```

## Quick Start

```toml
[dependencies]
merix-db = { workspace = true }
```

```rust
use merix_db::{MerixDbPool, init, apply_schemas};

#[tokio::main]
async fn main() -> Result<(), merix_core::MerixError> {
    // Simple default (standard connection)
    let db = init().await?;

    // Or use the full pool
    let pool = MerixDbPool::init().await?;
    let db = pool.standard();

    // Apply schemas once
    apply_schemas(db, &[
        "DEFINE TABLE IF NOT EXISTS users SCHEMALESS;",
        // ...
    ]).await?;

    Ok(())
}
```

## Document Operations

```rust
use merix_db::document;

let rid = document::insert(db, "users", user_data).await?;
let user: Option<User> = document::find_by_id(db, &rid).await?;
```

## Vector Search

```rust
use merix_db::vector_search;

vector_search::upsert(db, "vectors", items, None).await?;           // auto IDs
let results = vector_search::search(db, "vectors", query, None).await?;
```

## Graph Operations

```rust
use merix_db::graph;

graph::create_edge(db, &alice_id, "follows", &bob_id, None).await?;
let friends = graph::traverse(db, &alice_id, GraphDirection::Out, "follows", Some(2), None).await?;
```

## Full-Text Search

```rust
use merix_db::full_text_search;

let results = full_text_search::search_text(db, "docs", "content", "rust ai", 20).await?;
```

## Geospatial Queries

```rust
use merix_db::geospatial;

let nearby = geospatial::nearby(db, "places", "location", (-112.0740, 33.4484), 5000.0, 20).await?;
```

## Time-Series Queries

```rust
use merix_db::time_series;

let latest = time_series::latest(db, "events", "timestamp", 50).await?;
let window = time_series::range(db, "events", "timestamp", start, end, Some(100)).await?;
```

## Best Practices

- Use `standard` for most application data and user records.
- Use `temporal` for versioned AI memory, history, or temporal graphs.
- Use `ephemeral` for hot runtime caches, tests, or temporary state.
- Define indexes once using the provided helpers (they are idempotent).
- Pre-compute embeddings before calling `vector_search::upsert`.
- Keep the db crate domain-agnostic — business logic lives in `merix-memory`, `merix-agent`, etc.

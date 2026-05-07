use surrealdb::engine::any::{connect as surreal_connect, Any};
use surrealdb::Surreal;
use std::sync::Arc;
use merix_core::{Config, MerixError};
use std::fs;
use tokio::time::{sleep, Duration};
use tracing;

/// Shared client (thread-safe + connection-reuse ready for heavy concurrent workloads)
pub type Db = Arc<Surreal<Any>>;

/// Connect to SurrealDB (embedded or remote) with smart retry logic.
pub async fn connect() -> Result<Db, MerixError> {
    let config = Config::load();

    let db_url = normalize_db_url(&config.db_url);

    // Ensure data dir exists for RocksDB-based URLs
    if let Some(path) = db_url.strip_prefix("rocksdb://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| MerixError::Db(format!("Failed to create data directory {}: {}", parent.display(), e)))?;
        }
    }

    // Embedded = no network → fail fast. Remote = allow retries for transient network/KV issues.
    let is_embedded = db_url.starts_with("rocksdb://")
        || db_url.starts_with("memory")
        || db_url.starts_with("mem://")
        || db_url.starts_with("surrealdb://") // future-proof for surrealkv etc.
        || db_url.starts_with("file://");

    let max_attempts = if is_embedded { 1 } else { 5 };
    let mut attempts = 0;

    let db: Surreal<Any> = loop {
        match surreal_connect(&db_url).await {
            Ok(db) => break db,
            Err(e) if attempts < max_attempts => {
                attempts += 1;
                tracing::warn!(
                    "DB connect attempt {}/{} failed ({}): {}",
                    attempts,
                    max_attempts,
                    if is_embedded { "embedded" } else { "remote" },
                    e
                );
                if attempts < max_attempts {
                    sleep(Duration::from_millis(300 * attempts as u64)).await;
                }
            }
            Err(e) => return Err(MerixError::Db(format!("Failed to connect to SurrealDB: {}", e))),
        }
    };

    db.use_ns("merix").use_db("main").await
        .map_err(|e| MerixError::Db(format!("Failed to select namespace/database: {}", e)))?;

    tracing::info!("Connected to SurrealDB v3 at {}", db_url);
    Ok(Arc::new(db))
}

/// Normalize common URL variants so `any::connect` is happy.
fn normalize_db_url(url: &str) -> String {
    let url = url.trim();
    if url.starts_with("memory:") || url == "memory" || url.starts_with("mem:") {
        "memory".to_string()
    } else {
        url.to_string()
    }
}
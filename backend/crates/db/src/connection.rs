use surrealdb::engine::any::{connect as surreal_connect, Any};
use surrealdb::Surreal;
use std::sync::Arc;
use merix_core::{Config, MerixError};
use std::fs;
use tokio::time::{sleep, Duration};
use tracing;

/// Shared client (thread-safe + connection-reuse ready for heavy concurrent workloads)
pub type Db = Arc<Surreal<Any>>;

/// Connect to embedded SurrealDB (RocksDB or in-memory) with retry logic for production stability.
pub async fn connect() -> Result<Db, MerixError> {
    let config = Config::load();

    let db_url = if config.db_url.starts_with("memory:") {
        "memory://".to_string()
    } else {
        config.db_url.clone()
    };

    // Ensure data dir exists (RocksDB)
    if let Some(path) = config.db_url.strip_prefix("rocksdb://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| MerixError::Db(format!("Failed to create data directory {}: {}", parent.display(), e)))?;
        }
    }

    // Production retry (transient network/KV issues under heavy load)
    let mut attempts = 0;
    let max_attempts = 5;
    let db: Surreal<Any> = loop {
        match surreal_connect(db_url.as_str()).await {
            Ok(db) => break db,
            Err(e) if attempts < max_attempts => {
                attempts += 1;
                tracing::warn!("DB connect attempt {}/{} failed: {}", attempts, max_attempts, e);
                sleep(Duration::from_millis(300 * attempts as u64)).await;
            }
            Err(e) => return Err(MerixError::Db(format!("Failed to connect to SurrealDB after retries: {}", e))),
        }
    };

    db.use_ns("merix").use_db("main").await
        .map_err(|e| MerixError::Db(format!("Failed to select namespace/database: {}", e)))?;

    tracing::info!("Connected to SurrealDB v3 at {}", db_url);
    Ok(Arc::new(db))
}
use surrealdb::engine::any::connect as surreal_connect;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use std::sync::Arc;
use merix_core::{Config, MerixError};
use std::fs;

pub type Db = Arc<Surreal<Any>>;

/// Connect to embedded SurrealDB (RocksDB or in-memory)
pub async fn connect() -> Result<Db, MerixError> {
    let config = Config::load();

    let db_url = if config.db_url.starts_with("memory:") {
        "memory://".to_string()
    } else {
        config.db_url.clone()
    };

    // Ensure the data directory exists before connecting (critical for RocksDB)
    if let Some(path) = config.db_url.strip_prefix("rocksdb://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| MerixError::Db(format!("Failed to create data directory {}: {}", parent.display(), e)))?;
            println!("✅ Created data directory: {}", parent.display());
        }
    }

    // Correct SurrealDB v3 Any engine connection
    let db: Surreal<Any> = surreal_connect(db_url.as_str())
        .await
        .map_err(|e| MerixError::Db(format!("Failed to connect to SurrealDB: {}", e)))?;

    db.use_ns("merix").use_db("main").await
        .map_err(|e| MerixError::Db(format!("Failed to select namespace/database: {}", e)))?;

    Ok(Arc::new(db))
}
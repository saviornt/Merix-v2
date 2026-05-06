use surrealdb::engine::any::connect as surreal_connect;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use std::sync::Arc;
use merix_core::{Config, MerixError};

/// Main database handle for Merix-V2
pub type Db = Arc<Surreal<Any>>;

/// Connect to embedded SurrealDB (RocksDB or in-memory)
pub async fn connect() -> Result<Db, MerixError> {
    let config = Config::load();

    let db_url = if config.db_url.starts_with("memory:") {
        "memory://".to_string()
    } else {
        config.db_url.clone()
    };

    // Correct SurrealDB v3 Any engine connection
    let db: Surreal<Any> = surreal_connect(db_url.as_str())
        .await
        .map_err(|e| MerixError::Db(format!("Failed to connect to SurrealDB: {}", e)))?;

    db.use_ns("merix").use_db("main").await
        .map_err(|e| MerixError::Db(format!("Failed to select namespace/database: {}", e)))?;

    Ok(Arc::new(db))
}

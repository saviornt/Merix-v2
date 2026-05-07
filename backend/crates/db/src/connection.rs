use surrealdb::engine::any::{connect as surreal_connect, Any};
use surrealdb::Surreal;
use std::sync::Arc;
use merix_core::Config;
use merix_core::MerixError;
use tracing;

/// Shared client type used by the entire db crate
pub type Db = Arc<Surreal<Any>>;

/// Low-level embedded connection factory for the three named engines.
pub(crate) mod engine {
    use super::*;

    pub async fn open_standard() -> Result<Db, MerixError> {
        let path = Config::standard_db_path();
        std::fs::create_dir_all(&path)
            .map_err(|e| MerixError::Db(format!("Failed to create standard_db directory: {}", e)))?;

        let url = format!("rocksdb://{}", path.to_string_lossy());
        connect_with_url(&url).await
    }

    pub async fn open_temporal() -> Result<Db, MerixError> {
        let path = Config::temporal_db_path();
        std::fs::create_dir_all(&path)
            .map_err(|e| MerixError::Db(format!("Failed to create temporal_db directory: {}", e)))?;

        let url = format!("surrealkv://{}", path.to_string_lossy());
        connect_with_url(&url).await
    }

    pub async fn open_ephemeral() -> Result<Db, MerixError> {
        connect_with_url("memory").await
    }

    /// Internal helper that always returns Surreal<Any>
    async fn connect_with_url(url: &str) -> Result<Db, MerixError> {
        let db: Surreal<Any> = surreal_connect(url)
            .await
            .map_err(|e| MerixError::Db(format!("Failed to connect to {}: {}", url, e)))?;

        db.use_ns("merix").use_db("main").await
            .map_err(|e| MerixError::Db(format!("Failed to select namespace/database: {}", e)))?;

        tracing::debug!("Connected to embedded database at {}", url);
        Ok(Arc::new(db))
    }
}
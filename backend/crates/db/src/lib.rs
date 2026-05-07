pub mod connection;
pub mod schemas;
pub mod operations;
pub mod vectors;

// Public API — everything other crates will import
pub use connection::{connect, Db};
pub use schemas::*;
pub use operations::*;
pub use vectors::*;

pub type MerixDb = Db;

/// One-call initialization for the entire Merix database layer.
/// Other crates should call this once at startup.
/// Now includes health check + schema init for production readiness.
pub async fn init() -> Result<MerixDb, merix_core::MerixError> {
    let db = connect().await?;
    // Production: verify connectivity under load
    db.health().await
        .map_err(|e| merix_core::MerixError::Db(format!("Health check failed: {}", e)))?;
    init_schemas(&db).await?;
    tracing::info!("Merix DB initialized successfully (SurrealDB v3, heavy-workload ready)");
    Ok(db)
}
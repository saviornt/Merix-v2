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

/// One-call initialization for the database layer.
/// Only handles connection + health check (production-ready, retries, tracing).
/// Schema initialization is now the caller's responsibility via `apply_schemas`
/// so the db crate stays completely domain-agnostic.
pub async fn init() -> Result<MerixDb, merix_core::MerixError> {
    let db = connect().await?;
    // Production: verify connectivity under load
    db.health().await
        .map_err(|e| merix_core::MerixError::Db(format!("Health check failed: {}", e)))?;

    tracing::info!("Merix DB layer initialized successfully (SurrealDB v3, heavy-workload ready — schemas handled by caller)");
    Ok(db)
}
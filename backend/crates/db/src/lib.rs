pub mod connection;
pub mod schemas;
pub mod document;
pub mod vector_search;
pub mod graph;
pub mod full_text_search;
pub mod geospatial;
pub mod time_series;

// Public API — everything other crates will import (kept stable)
pub use connection::connect;
pub use connection::Db;
pub use schemas::apply_schemas;     // ← required by the test
pub use schemas::*;
pub use document::*;
pub use vector_search::*;
pub use graph::*;
pub use full_text_search::*;
pub use geospatial::*;
pub use time_series::*;

// Recommended high-level initialization (connect + health check)
pub async fn init() -> Result<Db, merix_core::MerixError> {
    let db = connect().await?;

    // Production-ready health check
    db.health().await
        .map_err(|e| merix_core::MerixError::Db(format!("Health check failed: {}", e)))?;

    tracing::info!("Merix DB layer initialized successfully (SurrealDB v3 multi-model ready)");
    Ok(db)
}

pub type MerixDb = Db;
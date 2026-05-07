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
pub async fn init() -> Result<MerixDb, merix_core::MerixError> {
    let db = connect().await?;
    init_schemas(&db).await?;
    Ok(db)
}
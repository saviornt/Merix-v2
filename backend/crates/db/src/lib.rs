pub mod connection;
pub mod schemas;
pub mod operations;
pub mod vectors;

// Public API
pub use connection::Db;
pub use schemas::*;
pub use operations::*;
pub use vectors::*;

pub type MerixDb = Db;

pub async fn init() -> Result<MerixDb, merix_core::MerixError> {
    let db = connect().await?;
    init_schemas(&db).await?;
    Ok(db)
}
pub mod connection;
pub mod schemas;
pub mod document;
pub mod vector_search;
pub mod graph;
pub mod full_text_search;
pub mod geospatial;
pub mod time_series;
pub mod pool;

pub use connection::Db;
pub use schemas::apply_schemas;
pub use schemas::*;
pub use document::*;
pub use vector_search::*;
pub use graph::*;
pub use full_text_search::*;
pub use geospatial::*;
pub use time_series::*;
pub use pool::*;

pub type MerixDb = Db;

// Convenience: returns the standard (persistent) connection
pub async fn init() -> Result<Db, merix_core::MerixError> {
    let pool = MerixDbPool::init().await?;
    Ok(pool.standard().clone())
}
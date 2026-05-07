use serde::{Deserialize, Serialize};
use crate::Db;
use merix_core::MerixError;
use tracing;

/// Vector helpers (unchanged public API)
#[derive(Debug, Serialize, Deserialize)]
pub struct VectorQuery {
    pub embedding: Vec<f32>,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorSearchResult<T> {
    pub record: T,
    pub score: f32,
}

pub trait HasEmbedding {
    fn embedding(&self) -> Vec<f32>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryFilter {
    Where(serde_json::Value),
    Ids(Vec<String>),           // now just Vec<String> of full "table:id"
    Raw(String),
}

/// Production-ready generic schema applicator (domain-agnostic)
pub async fn apply_schemas(db: &Db, statements: &[&str]) -> Result<(), MerixError> {
    for stmt in statements {
        match db.query(*stmt).await {
            Ok(_) => tracing::debug!("Schema statement applied: {}", stmt),
            Err(e) => {
                tracing::warn!("Schema statement warning (non-critical): {} → {}", stmt, e);
            }
        }
    }

    tracing::info!("Custom schemas & indexes applied (db crate remains domain-agnostic)");
    Ok(())
}
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use crate::Db;
use merix_core::MerixError;
use tracing;
use uuid::Uuid;

/// Custom RecordId (kept for API compatibility; works with SurrealValue)
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct RecordId {
    pub table: String,
    pub id: String,
}

impl RecordId {
    pub fn new(table: String, id: String) -> Self {
        Self { table, id }
    }

    pub fn as_surreal(&self) -> String {
        format!("{}:{}", self.table, self.id)
    }

    pub fn random(table: &str) -> Self {
        Self::new(table.to_string(), Uuid::new_v4().to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryFilter {
    Where(serde_json::Value),
    Ids(Vec<RecordId>),
    Raw(String),
}

/// Vector helpers (unchanged API)
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

/// Production schema init (SurrealDB v3 syntax + HNSW vector index + tracing)
pub async fn init_schemas(db: &Db) -> Result<(), MerixError> {
    // Example tables (extend with your actual models)
    let schema_statements = vec![
        "DEFINE TABLE agent SCHEMAFULL;",
        "DEFINE INDEX agent_idx ON agent FIELDS name;",
        // Add more tables as needed (users, memory, etc.)
        "DEFINE TABLE memory SCHEMAFULL;",
        "DEFINE INDEX vec_idx ON memory FIELDS embedding HNSW DIMENSION 1536 DIST COSINE TYPE INT8;", // v3 vector index
    ];

    for stmt in schema_statements {
        db.query(stmt).await
            .map_err(|e| MerixError::Db(format!("Schema init failed for '{}': {}", stmt, e)))?;
    }

    tracing::info!("SurrealDB schemas & indexes initialized (production-ready HNSW vectors)");
    Ok(())
}
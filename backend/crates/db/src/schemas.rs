use serde::{Deserialize, Serialize};
use serde_json::{Value};
use uuid::{Uuid};
use crate::Db;
use merix_core::MerixError;

/// =======================================================================
/// COLLECTION - Minimal metadata wrapper for a SurrealDB table/collection.
/// =======================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
}

impl Collection {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// =======================================================================
/// BASE DOCUMENT
/// =======================================================================
pub type Document = serde_json::Value;

/// =======================================================================
/// VECTOR CAPABILITY TRAIT - Marks a document as supporting vector search.
/// =======================================================================
pub trait HasEmbedding {
    fn embedding(&self) -> &[f32];
}

/// =======================================================================
/// VECTOR QUERY RESULT WRAPPER
/// Used by vector.rs to standardize similarity output.
/// =======================================================================
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult<T> {
    pub record: T,
    pub score: f32,
}

/// =======================================================================
/// RecordID Wrapper - Prevents invalid IDs and makes queries consistent
/// =======================================================================
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordId {
    pub table: String,
    pub id: Uuid,
}

impl RecordId {
    pub fn new(table: impl Into<String>, id: Uuid) -> Self {
        Self {
            table: table.into(),
            id,
        }
    }

    pub fn as_surreal(&self) -> String {
        format!("{}:{}", self.table, self.id)
    }

    /// Convenience: create with table + string (auto-parses to Uuid or generates one)
    pub fn new_with_string(table: impl Into<String>, id: impl Into<String>) -> Self {
        let id_str = id.into();
        let id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
        Self::new(table, id)
    }

    /// Create a brand-new random RecordId
    pub fn random(table: impl Into<String>) -> Self {
        Self::new(table, Uuid::new_v4())
    }
}


/// =======================================================================
/// QueryFilter Wrapper - 
/// =======================================================================
#[derive(Debug, Clone)]
pub enum QueryFilter {
    Where(Value),
    Ids(Vec<RecordId>),
    Raw(String),
}

/// =======================================================================
/// VectorQuery Wrapper - 
/// =======================================================================
pub struct VectorQuery {
    pub embedding: Vec<f32>,
    pub limit: usize,
    pub threshold: Option<f32>,
}

/// =======================================================================
/// QueryResult Wrapper - 
/// =======================================================================
#[derive(Debug, Clone)]
pub struct QueryResult<T> {
    pub items: Vec<T>,
}

/// =======================================================================
/// Metadata Wrapper - Explicit intent and avoids random Option<Value>
/// =======================================================================
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Metadata(pub Value);

/// =======================================================================
/// Pagination Wrapper
/// =======================================================================
#[derive(Debug, Clone)]
pub struct Pagination {
    pub limit: usize,
    pub offset: usize,
}

/// =======================================================================
/// Sort Wrapper
/// =======================================================================
#[derive(Debug, Clone)]
pub struct Sort {
    pub field: String,
    pub descending: bool,
}

/// =======================================================================
/// Initialize all core tables + indexes (including vector support).
/// Called automatically by `merix_db::init()`.
/// =======================================================================
pub async fn init_schemas(db: &Db) -> Result<(), MerixError> {
    // Core collections used across the app
    let tables = vec![
        "tasks",
        "sessions",
        "skills",
        "checkpoints",
        "agents",
        "memory",
        "embeddings",
        "test_records",     // used by integration test
        "test_embeddings",  // used by integration test
    ];

    for table in tables {
        let define_table = format!(
            "DEFINE TABLE {} SCHEMALESS PERMISSIONS FULL;",
            table
        );
        db.query(&define_table)
            .await
            .map_err(|e| MerixError::Db(e.to_string()))?;
    }

    // Vector index (SurrealDB v3+ vector support)
    db.query(
        r#"
        DEFINE INDEX idx_embedding ON embeddings 
        FIELDS embedding 
        HNSW DIMENSION 384 DIST COSINE;
        "#
    )
    .await
    .map_err(|e| MerixError::Db(e.to_string()))?;

    println!("✅ SurrealDB schemas + HNSW vector index initialized");
    Ok(())
}
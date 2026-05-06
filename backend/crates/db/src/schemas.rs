use serde::{Deserialize, Serialize};
use serde_json::{Value};

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
    pub id: String,
}

impl RecordId {
    pub fn new(table: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            id: id.into(),
        }
    }

    pub fn as_surreal(&self) -> String {
        format!("{}:{}", self.table, self.id)
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
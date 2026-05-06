use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
}

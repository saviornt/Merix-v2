use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub code: String,
    pub embedding: Vec<f32>,
    pub version: String,
    pub created_at: DateTime<Utc>,
}

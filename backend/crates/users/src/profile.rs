use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub preferences: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

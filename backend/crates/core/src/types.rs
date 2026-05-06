use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Core shared types for the entire Merix-V2 application
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub id: Uuid,
    pub model: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub content: String,
    pub embedding: Vec<f32>,
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub code: String,
    pub embedding: Vec<f32>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCall {
    pub tool_name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentState {
    Idle,
    Thinking,
    RunningTool(String),
    WaitingForUser,
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_session_serialization_roundtrip() {
        let session = Session {
            id: Uuid::new_v4(),
            model: "llama3.2:3b".to_string(),
            created_at: Utc::now(),
        };
        let json = serde_json::to_string(&session).unwrap();
        let deserialized: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(session, deserialized);
    }

    #[test]
    fn test_message_serialization_roundtrip() {
        let msg = Message {
            role: "user".to_string(),
            content: "Hello, Merix!".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_memory_entry_serialization_roundtrip() {
        let entry = MemoryEntry {
            id: Uuid::new_v4(),
            content: "Test memory".to_string(),
            embedding: vec![0.1, 0.2, 0.3],
            timestamp: Utc::now(),
            tags: vec!["test".to_string()],
        };
        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: MemoryEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry.id, deserialized.id);
        assert_eq!(entry.content, deserialized.content);
    }
}

use serde::{Deserialize, Serialize};
use surrealdb::Error as SurrealError;
use serde_json::Error as SerdeError;
use thiserror::Error;

impl From<SerdeError> for MerixError {
    fn from(e: SerdeError) -> Self {
        MerixError::Db(e.to_string())
    }
}

impl From<SurrealError> for MerixError {
    fn from(e: SurrealError) -> Self {
        MerixError::Db(e.to_string())
    }
}

/// Central error type for the entire Merix-V2 application
#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MerixError {
    #[error("Inference error: {0}")]
    Inference(String),

    #[error("Database error: {0}")]
    Db(String),

    #[error("Agent error: {0}")]
    Agent(String),

    #[error("Tool error: {0}")]
    Tool(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_error_serialization_roundtrip() {
        let errors = vec![
            MerixError::Inference("model failed to load".to_string()),
            MerixError::Db("failed to connect to SurrealDB".to_string()),
            MerixError::Agent("skill execution failed".to_string()),
            MerixError::Tool("file read permission denied".to_string()),
        ];

        for err in errors {
            let json = serde_json::to_string(&err).unwrap();
            let deserialized: MerixError = serde_json::from_str(&json).unwrap();
            assert_eq!(err, deserialized);
        }
    }
}

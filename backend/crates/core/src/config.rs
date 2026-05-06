use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::MerixError;

/// Global configuration for Merix-V2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Directory where GGUF models are stored (default: ./models)
    pub model_dir: PathBuf,

    /// SurrealDB connection URL (rocksdb://./data/merix.db or memory://)
    pub db_url: String,

    /// Default LLM model to load on startup
    pub default_model: String,

    /// Default Whisper model for speech-to-text
    pub default_whisper_model: String,

    /// Maximum number of tokens in context window
    pub max_context_tokens: usize,

    /// Whether to start the embedded OpenAI-compatible server (localhost:11434)
    pub enable_ollama_server: bool,

    /// Port for the embedded OpenAI server
    pub server_port: u16,

    /// Enable agent self-improvement loop
    pub enable_self_improvement: bool,

    /// Number of background agent workers
    pub agent_workers: usize,
}

impl Config {
    /// Load config from TOML (or return defaults if file doesn't exist)
    pub fn load() -> Self {
        // TODO: In a later phase we'll add real TOML file loading from ~/.merix/config.toml
        Self::default()
    }

    /// Save config to TOML file
    pub fn save(&self) -> Result<(), MerixError> {
        // Placeholder for future TOML serialization
        Ok(())
    }

    /// Returns the default configuration
    pub fn default_model_dir() -> PathBuf {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("models")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_dir: Self::default_model_dir(),
            db_url: "rocksdb://./data/merix.db".to_string(),
            default_model: "llama3.2:3b".to_string(),
            default_whisper_model: "whisper-tiny".to_string(),
            max_context_tokens: 8192,
            enable_ollama_server: true,
            server_port: 11434,
            enable_self_improvement: true,
            agent_workers: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = Config {
            model_dir: PathBuf::from("./test-models"),
            db_url: "memory://".to_string(),
            default_model: "phi3:mini".to_string(),
            default_whisper_model: "whisper-base".to_string(),
            max_context_tokens: 4096,
            enable_ollama_server: false,
            server_port: 11435,
            enable_self_improvement: false,
            agent_workers: 2,
        };

        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_config_default_values() {
        let config = Config::default();
        assert_eq!(config.default_model, "llama3.2:3b");
        assert_eq!(config.max_context_tokens, 8192);
        assert!(config.enable_ollama_server);
    }
}

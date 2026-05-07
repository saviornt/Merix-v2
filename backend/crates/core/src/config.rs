use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::MerixError;

/// Global configuration for Merix-V2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Directory where GGUF models are stored
    pub model_dir: PathBuf,

    /// SurrealDB connection URL (auto-computed rocksdb path)
    pub db_url: String,

    /// Default LLM model to load on startup
    pub default_model: String,

    /// Default Whisper model for speech-to-text
    pub default_whisper_model: String,

    /// Maximum number of tokens in context window
    pub max_context_tokens: usize,

    /// Whether to start the embedded OpenAI-compatible server
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

    /// Returns the intelligent data directory (dev vs production)
    pub fn data_dir() -> PathBuf {
        // Developer mode detection (works during cargo test, cargo run, cargo tauri dev)
        if std::env::var_os("CARGO_MANIFEST_DIR").is_some() || cfg!(debug_assertions) {
            let manifest = std::env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| ".".to_string());
            let mut root = PathBuf::from(manifest);

            // Walk up from backend/crates/core → project root
            root.pop(); // core
            root.pop(); // crates
            root.pop(); // backend
            root.pop(); // project root (Merix-v2/)

            root.join("test_data")
        } else {
            // Production (packaged Tauri app) → proper OS app data directory
            dirs::data_dir()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
                .join("Merix")
                .join("db")
        }
    }

    /// Returns the default model directory (keeps existing behavior for models)
    pub fn default_model_dir() -> PathBuf {
            let data_dir = Self::data_dir();
            let parent = data_dir.parent().unwrap_or(&data_dir);
            parent.join("models")
    }
}

impl Default for Config {
    fn default() -> Self {
        let data_dir = Self::data_dir();
        let db_path = data_dir.join("merix.db");

        Self {
            model_dir: Self::default_model_dir(),
            // ✅ Smart rocksdb URL — this is what the db crate now uses
            db_url: format!("rocksdb://{}", db_path.to_string_lossy()),
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

    #[test]
    fn test_config_serialization_roundtrip() { /* ... unchanged ... */ }

    #[test]
    fn test_config_default_values() { /* ... unchanged ... */ }

    #[test]
    fn test_data_dir_dev_vs_prod() {
        let dir = Config::data_dir();
        println!("Data dir resolved to: {:?}", dir);
        // In CI/dev it should contain "test_data"
    }
}
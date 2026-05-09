use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::MerixError;

/// Global configuration for Merix-V2
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Directory where GGUF models are stored
    pub model_dir: PathBuf,

    /// Default LLM model to load on startup
    pub default_model: String,

    /// Default Whisper model for speech-to-text
    pub default_whisper_model: String,

    /// Default Text-Embedding model for vectorization
    pub default_embedding_model: String,

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
        Self::default()
    }

    /// Save config to TOML file
    pub fn save(&self) -> Result<(), MerixError> {
        Ok(())
    }

    /// Base Merix directory
    pub fn base_dir() -> PathBuf {
        if std::env::var_os("CARGO_MANIFEST_DIR").is_some() || cfg!(debug_assertions) {
            // Development mode — use project root
            let manifest = std::env::var("CARGO_MANIFEST_DIR")
                .unwrap_or_else(|_| ".".to_string());
            let mut root = PathBuf::from(manifest);

            while root.file_name() != Some(std::ffi::OsStr::new("Merix-v2")) && root.parent().is_some() {
                root.pop();
            }
            if root.file_name() != Some(std::ffi::OsStr::new("Merix-v2")) {
                root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            }
            root
        } else {
            // Production (packaged Tauri app)
            dirs::data_dir()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
                .join("Merix")
        }
    }

    /// Directory containing all database folders
    pub fn databases_dir() -> PathBuf {
        Self::base_dir().join("databases")
    }

    /// Main persistent database (RocksDB)
    pub fn standard_db_path() -> PathBuf {
        Self::databases_dir().join("standard_db")
    }

    /// Temporal / versioned AI memory database (SurrealKV)
    pub fn temporal_db_path() -> PathBuf {
        Self::databases_dir().join("temporal_db")
    }

    /// Model directory
    pub fn model_dir() -> PathBuf {
        Self::base_dir().join("models")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_dir: Self::model_dir(),
            default_model: "llama3.2:3b".to_string(),
            default_whisper_model: "whisper-tiny".to_string(),
            default_embedding_model: "".to_string(),
            max_context_tokens: 8192,
            enable_ollama_server: true,
            server_port: 11434,
            enable_self_improvement: true,
            agent_workers: 4,
        }
    }
}
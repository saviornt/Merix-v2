// backend/crates/inference/src/pool_embedders.rs
//! High-level embedding pool for Merix-v2 (AI Package Manager).
//!
//! Main public API for other crates:
//! ```rust
//! let vector = inference::generate_embedding(text, model_name).await?;
//! ```

use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use async_trait::async_trait;
use merix_core::MerixError;
use tokio::sync::Mutex;

use crate::embedders::candle_embed::CandleEmbedder;
use crate::embedders::llama_embed::LlamaEmbedder;

/// Global embedder pool (lazily initialized and protected by Mutex).
static EMBEDDER_POOL: OnceLock<Mutex<EmbedderPool>> = OnceLock::new();

/// High-level embedding pool.
pub struct EmbedderPool {
    embedders: HashMap<String, Arc<dyn Embedder + Send + Sync>>,
}

impl EmbedderPool {
    pub fn new() -> Self {
        Self {
            embedders: HashMap::new(),
        }
    }

    /// Get or lazily create the appropriate embedder for the given model name.
    async fn get_or_create(&mut self, model_name: &str) -> Result<Arc<dyn Embedder + Send + Sync>, MerixError> {
        if let Some(embedder) = self.embedders.get(model_name) {
            return Ok(embedder.clone());
        }

        // Intelligent backend selection
        let embedder: Arc<dyn Embedder + Send + Sync> = if model_name.ends_with(".gguf") {
            Arc::new(LlamaEmbedder::new(model_name)?)
        } else {
            Arc::new(CandleEmbedder::new(model_name)?)
        };

        self.embedders.insert(model_name.to_string(), embedder.clone());

        Ok(embedder)
    }
}

/// High-level convenience function — this is what other crates will call.
pub async fn generate_embedding(
    text: impl Into<String>,
    model_name: impl AsRef<str>,
) -> Result<Vec<f32>, MerixError> {
    let pool = EMBEDDER_POOL.get_or_init(|| Mutex::new(EmbedderPool::new()));
    let mut guard = pool.lock().await;
    guard.get_or_create(model_name.as_ref()).await?.embed(&text.into()).await
}

/// **Native async** embedder trait (re-exported as `crate::Embedder`).
#[async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError>;
}
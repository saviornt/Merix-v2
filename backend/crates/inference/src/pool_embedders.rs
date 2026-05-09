// backend/crates/inference/src/pool_embedders.rs
//! High-level embedding pool for Merix-v2.
//!
//! Main public API for other crates:
//! ```rust
//! let vector = inference::generate_embedding(text, model_name).await?;
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

use async_trait::async_trait;
use merix_core::MerixError;

/// Global embedder pool (initialized once).
static EMBEDDER_POOL: OnceLock<EmbedderPool> = OnceLock::new();

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

    pub fn register(&mut self, model_name: impl Into<String>, embedder: Arc<dyn Embedder + Send + Sync>) {
        self.embedders.insert(model_name.into(), embedder);
    }

    pub async fn generate_embedding(
        &self,
        text: impl Into<String>,
        model_name: impl AsRef<str>,
    ) -> Result<Vec<f32>, MerixError> {
        let model_name = model_name.as_ref();
        let embedder = self.embedders.get(model_name)
            .ok_or_else(|| MerixError::Inference(format!("No embedder registered for model: {model_name}")))?;

        embedder.embed(&text.into()).await
    }
}

/// High-level convenience function — this is what other crates will call.
pub async fn generate_embedding(
    text: impl Into<String>,
    model_name: impl AsRef<str>,
) -> Result<Vec<f32>, MerixError> {
    let pool = EMBEDDER_POOL.get_or_init(EmbedderPool::new);
    pool.generate_embedding(text, model_name).await
}

/// **Native async** embedder trait (re-exported as `crate::Embedder`).
#[async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError>;
}
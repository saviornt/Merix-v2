// backend/crates/inference/src/embedders/llama_embed/llama_embedder.rs
//! oxillama-backed text embedder using GGUF models for Merix-v2 (llama.cpp backend).
//!
//! Uses the exact same inference engine as the LLM/STT stack for maximum
//! consistency, minimal binary size, and shared device/quantization logic.

use std::path::Path;
use std::sync::Mutex;

use merix_core::MerixError;
use oxillama::runtime::{EngineConfig, InferenceEngine, PoolingMode};

use crate::Embedder;

/// Llama.cpp (GGUF) embedder for Merix-v2.
pub struct LlamaEmbedder {
    // Mutex because `embed_with` requires `&mut self`.
    engine: Mutex<InferenceEngine>,
}

impl LlamaEmbedder {
    /// Create a new embedder from a GGUF model path.
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self, MerixError> {
        let model_path_str = model_path.as_ref().to_string_lossy().into_owned();

        let config = EngineConfig {
            model_path: model_path_str,
            ..Default::default()
        };

        let mut engine = InferenceEngine::new(config);
        engine
            .load_model()
            .map_err(|e| MerixError::Inference(format!("failed to load GGUF model: {e}")))?;

        Ok(Self {
            engine: Mutex::new(engine),
        })
    }
}

#[async_trait::async_trait]
impl Embedder for LlamaEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError> {
        let mut guard = self.engine.lock().map_err(|poisoned| {
            MerixError::Inference(format!("embedder mutex poisoned: {poisoned}"))
        })?;

        guard
            .embed_with(text, PoolingMode::Mean)
            .map_err(|e| MerixError::Inference(format!("embedding failed: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MODEL: &str = "../../../models/nomic-embed-text-v1.5.gguf";

    #[tokio::test]
    async fn test_llama_embedder_new_and_embed() {
        if !std::path::Path::new(TEST_MODEL).exists() {
            eprintln!("Skipping embed test: test model not found at {TEST_MODEL}");
            return;
        }

        let embedder = LlamaEmbedder::new(TEST_MODEL)
            .expect("failed to create LlamaEmbedder with test model");

        let embedding = embedder
            .embed("Merix-v2 is an agentic operating system for AI.")
            .await
            .expect("embed call failed");

        assert_eq!(embedding.len(), 768, "embedding dimension mismatch");
        assert!(!embedding.is_empty());

        let norm: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01, "embedding not properly normalized");
    }
}
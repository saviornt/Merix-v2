// backend/crates/inference/src/embed/text_embedder.rs
//! oxillama-backed text embedder using GGUF models for Merix-v2.
//!
//! Uses the exact same inference engine as the LLM/STT stack for maximum
//! consistency, minimal binary size, and shared device/quantization logic.

use std::path::Path;
use std::sync::Mutex;

use merix_core::MerixError;
use oxillama::runtime::{EngineConfig, InferenceEngine, PoolingMode};

use crate::text_embed::traits::Embedder;

/// Text embedder for Merix-v2 using llama.cpp-compatible GGUF embedding models.
///
/// # Model notes
/// - Compatible with `nomic-embed-text-v1.5.gguf` (or any embedding GGUF).
/// - Model is **user-provided** — never shipped with the binary.
/// - Supports CPU + Metal + CUDA via oxillama’s device selection (inherited from `EngineConfig`).
pub struct TextEmbedder {
    // Mutex because `embed_with` (and `load_model`) require `&mut self`.
    // This keeps the public `Embedder` trait as `&self` for easy sharing across threads.
    engine: Mutex<InferenceEngine>,
}

impl TextEmbedder {
    /// Create a new text embedder from a GGUF model path.
    ///
    /// # Arguments
    /// * `model_path` — Path to the `.gguf` file (e.g. `"models/nomic-embed-text-v1.5.gguf"`).
    ///
    /// # Errors
    /// Returns `MerixError` if the model cannot be loaded or the engine fails to initialize.
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

impl Embedder for TextEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError> {
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

    /// Path to the test model (same pattern used by the rest of the inference crate).
    const TEST_MODEL: &str = "../../models/nomic-embed-text-v1.5.gguf";

    #[test]
    fn test_text_embedder_new_and_embed() {
        // Skip if test model is not present (common pattern in Merix-v2 tests).
        if !std::path::Path::new(TEST_MODEL).exists() {
            eprintln!("Skipping embed test: test model not found at {TEST_MODEL}");
            return;
        }

        let embedder = TextEmbedder::new(TEST_MODEL)
            .expect("failed to create TextEmbedder with test model");

        let embedding = embedder
            .embed("Merix-v2 is an agentic operating system for AI.")
            .expect("embed call failed");

        // nomic-embed-text-v1.5 produces 768-dimensional embeddings
        assert_eq!(embedding.len(), 768, "embedding dimension mismatch");
        assert!(!embedding.is_empty(), "embedding vector is empty");

        // Quick sanity check: vector should be normalized (L2 norm ≈ 1.0)
        let norm: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01, "embedding not properly normalized");
    }
}
// backend/crates/inference/src/embedders/llama_embed/traits.rs
//! Async trait for Llama.cpp (GGUF) embedder.

use merix_core::MerixError;

#[async_trait::async_trait]
pub trait LlamaEmbedderTraits: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError>;
}
// backend/crates/inference/src/embedders/candle_embed/traits.rs
//! Async trait for Candle embedder (local-only).

use merix_core::MerixError;

#[async_trait::async_trait]
pub trait CandleEmbedderTraits: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError>;
}
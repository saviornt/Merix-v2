// backend/crates/inference/src/embedders/candle_embed/mod.rs
//! Candle embedder implementation.

pub mod traits;
pub mod candle_embedder;

pub use traits::CandleEmbedderTraits;
pub use candle_embedder::CandleEmbedder;
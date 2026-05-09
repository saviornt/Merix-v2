// backend/crates/inference/src/embedders/mod.rs
//! All concrete embedder implementations for Merix-v2.
//!
//! This is the home for Candle, GGUF (TextEmbedder), and any future embedder backends.
//! Other crates should prefer using the high-level `EmbedderPool` when possible.

pub mod candle_embed;
pub mod llama_embed;

// Re-export the main types for convenience
pub use candle_embed::CandleEmbedder;
pub use llama_embed::LlamaEmbedder;
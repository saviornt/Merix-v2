// backend/crates/inference/src/lib.rs
//! Core inference primitives for Merix-v2.

pub mod llm;
pub mod stt;
pub mod server;

// Model acquisition / package manager
pub mod hf_downloader;

// Specialized pools (Phase 5+ architecture)
pub mod pool_embedders;

// Embedders live under their own submodule
pub mod embedders;

// Re-exports for convenience
pub use hf_downloader::HfDownloader;
pub use embedders::{ CandleEmbedder, LlamaEmbedder };
pub use pool_embedders::{generate_embedding, Embedder};
// backend/crates/inference/src/embed/mod.rs
//! Text embedding module for Merix-v2 RAG pipeline.
//!
//! Provides the `Embedder` trait and a production-ready `TextEmbedder` backed by
//! oxillama + GGUF models (e.g. `nomic-embed-text-v1.5.gguf`).

pub mod traits;
pub mod llama_embedder;

pub use traits::LlamaEmbedderTraits;
pub use llama_embedder::LlamaEmbedder;
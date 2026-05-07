// backend/crates/inference/src/embed/mod.rs
//! Text embedding module for Merix-v2 RAG pipeline.
//!
//! Provides the `Embedder` trait and a production-ready `TextEmbedder` backed by
//! oxillama + GGUF models (e.g. `nomic-embed-text-v1.5.gguf`).
//!
//! Models are **not** shipped with the app — users download them and pass the
//! local path at construction time (same pattern as the LLM).

pub mod traits;
pub mod text_embedder;

pub use traits::Embedder;
pub use text_embedder::TextEmbedder;
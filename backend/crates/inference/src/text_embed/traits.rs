// backend/crates/inference/src/embed/trait_.rs
//! The core `Embedder` trait for Merix-v2.
//!
//! All embedding backends must implement this minimal, high-performance API.

use merix_core::MerixError;

/// Produces dense vector embeddings for text (used by RAG, semantic search, etc.).
pub trait Embedder {
    /// Generate a normalized embedding vector for the input text.
    ///
    /// # Errors
    /// Returns `MerixError` if model loading failed, tokenization failed,
    /// or inference encountered an error.
    fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError>;
}
// backend/crates/inference/src/candle_embed/mod.rs
//! Candle-based text embedder backend for Merix-v2.
//!
//! This is the production implementation of the `Embedder` trait using
//! `candle-transformers::models::bert`.

pub mod candle_embedder;

pub use candle_embedder::CandleEmbedder;
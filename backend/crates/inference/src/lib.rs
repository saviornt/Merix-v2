// backend/crates/inference/src/lib.rs
//! Core inference primitives for Merix-v2.
//!
//! This crate provides LLM, STT, and embedding capabilities using a consistent
//! llama.cpp-based stack via the `oxillama`.

pub mod llm;
pub mod stt;
pub mod server;
pub mod text_embed;
pub mod candle_embed;

pub use text_embed::{ Embedder, TextEmbedder };
pub use candle_embed::{ CandleEmbedder };
// backend/crates/inference/src/hf_downloader/mod.rs
//! Model acquisition system (AI Package Manager) for Merix-v2.
//!
//! This is the **only** module other crates should import and use.
//! It hides all Hugging Face internals, file formats, and low-level details.

pub mod downloader;

pub use downloader::{HfDownloader, AcquiredModel};
// backend/crates/inference/tests/test_embed.rs
//! Integration tests for the public `Embedder` API (Merix-v2 inference crate).
//!
//! These tests exercise the full public surface (`TextEmbedder` + `Embedder` trait)
//! exactly as downstream crates (rag, agents, memory, etc.) will use it.
//!
//! Run with `--nocapture` to see the actual embedding vector printed to the console.

use merix_inference::{Embedder, TextEmbedder};

#[test]
fn integration_text_embedder_smoke_test() {
    const TEST_MODEL: &str = "../../../models/bge-small-en-v1.5.gguf";

    // Graceful skip if model is missing (CI / fresh clone friendly).
    if !std::path::Path::new(TEST_MODEL).exists() {
        eprintln!("Skipping integration embed test: test model not found at {TEST_MODEL}");
        return;
    }
    eprint!("Loading model: {TEST_MODEL}");
    let embedder =
        TextEmbedder::new(TEST_MODEL).expect("failed to load TextEmbedder in integration test");

    let embedding = embedder
        .embed("Testing Merix-v2 embedding integration - this should produce a real vector")
        .expect("embed() failed in integration test");

    // === VISUAL OUTPUT FOR YOU ===
    println!("\n=== EMBEDDING DEMO OUTPUT ===");
    println!(
        "Sample text: \"Testing Merix-v2 embedding integration - this should produce a real vector\""
    );
    println!("Embedding length (dimension): {}", embedding.len());
    println!("First 10 values: {:?}", &embedding[0..10]);

    // Quick sanity check (L2 norm should be ≈ 1.0 for normalized embeddings)
    let norm: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
    println!("L2 norm (should be ≈ 1.0): {:.6}", norm);
    println!("=============================\n");

    assert_eq!(
        embedding.len(),
        768,
        "integration test: wrong embedding dimension"
    );
    assert!(!embedding.is_empty());
}

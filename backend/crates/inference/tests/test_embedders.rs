// backend/crates/inference/tests/test_embedders.rs
//! Integration tests for the concrete embedder implementations (Merix-v2 inference crate).
//!
//! Tests both Candle and Llama (GGUF) embedders.
//! Run with `--nocapture` to see the embedding vectors.

use std::sync::Arc;

use merix_inference::{CandleEmbedder, embedders::llama_embed::LlamaEmbedder, pool_embedders::Embedder};

#[tokio::test]
async fn integration_candle_embedder_smoke_test() {
    const TEST_MODEL_NAME: &str = "sentence-transformers/all-MiniLM-L6-v2";
    const TEST_TEXT: &str = "Testing Merix-v2 Candle embedder directly";

    println!("\n=== CANDLE EMBEDDER DIRECT TEST ===");
    println!("Model : {}", TEST_MODEL_NAME);
    println!("Text  : {}", TEST_TEXT);

    let embedder = Arc::new(
        CandleEmbedder::new(TEST_MODEL_NAME)
            .expect("failed to create CandleEmbedder")
    );

    let vector = embedder.embed(TEST_TEXT)
        .await
        .expect("embed call failed");

    // === VISUAL OUTPUT ===
    println!("\n=== EMBEDDING RESULT ===");
    println!("Dimension : {}", vector.len());
    println!("First 10 values: {:?}", &vector[0..10]);
    let norm: f32 = vector.iter().map(|&x| x * x).sum::<f32>().sqrt();
    println!("L2 norm (should be ≈ 1.0): {:.6}", norm);
    println!("=============================\n");

    assert_eq!(vector.len(), 384, "wrong embedding dimension");
    assert!(!vector.is_empty());
}

/// LlamaEmbedder (GGUF) test is currently ignored because oxillama does not yet
/// support bert/nomic-bert architectures reliably.
/// Re-enable by removing `#[ignore]` once GGUF embedding support is stable.
#[ignore]
#[tokio::test]
async fn integration_llama_embedder_smoke_test() {
    const TEST_MODEL_NAME: &str = "nomic-embed-text-v1.5.gguf"; // or whatever your GGUF filename is
    const TEST_TEXT: &str = "Testing Merix-v2 Llama (GGUF) embedder directly";

    println!("\n=== LLAMA EMBEDDER (GGUF) DIRECT TEST ===");
    println!("Model : {}", TEST_MODEL_NAME);
    println!("Text  : {}", TEST_TEXT);

    let embedder = Arc::new(
        LlamaEmbedder::new(TEST_MODEL_NAME)
            .expect("failed to create LlamaEmbedder")
    );

    let vector = embedder.embed(TEST_TEXT)
        .await
        .expect("embed call failed");

    // === VISUAL OUTPUT ===
    println!("\n=== EMBEDDING RESULT ===");
    println!("Dimension : {}", vector.len());
    println!("First 10 values: {:?}", &vector[0..10]);
    let norm: f32 = vector.iter().map(|&x| x * x).sum::<f32>().sqrt();
    println!("L2 norm (should be ≈ 1.0): {:.6}", norm);
    println!("=============================\n");

    assert!(!vector.is_empty());
}
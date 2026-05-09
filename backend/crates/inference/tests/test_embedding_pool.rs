// backend/crates/inference/tests/test_embedding_pool.rs
//! Integration tests for the high-level embedding pool API (Merix-v2 inference crate).
//!
//! Demonstrates the public API other crates will use:
//! ```rust
//! inference::initialize_default_embedder("model-name").await?;
//! let vector = inference::generate_embedding(text, model_name).await?;
//! ```

use merix_inference::{generate_embedding, CandleEmbedder, pool_embedders::EmbedderPool};

#[tokio::test]
async fn integration_embedding_pool_smoke_test() {
    const TEST_MODEL_NAME: &str = "sentence-transformers/all-MiniLM-L6-v2";
    const TEST_TEXT: &str = "Testing Merix-v2 high-level embedding pool API";

    println!("\n=== EMBEDDING POOL HIGH-LEVEL API TEST ===");
    println!("Model : {}", TEST_MODEL_NAME);
    println!("Text  : {}", TEST_TEXT);

    // One-time initialization (this is the KISS pattern we want)
    // In production this would be called once at app startup
    CandleEmbedder::register_default(TEST_MODEL_NAME)
        .await
        .expect("failed to register default embedder");

    let vector = generate_embedding(TEST_TEXT, TEST_MODEL_NAME)
        .await
        .expect("generate_embedding failed");

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
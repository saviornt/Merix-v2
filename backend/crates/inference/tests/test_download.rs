// backend/crates/inference/tests/test_download.rs
//! Integration tests for the high-level model acquisition system (hf_downloader).
//!
//! This test simply calls the public API that other crates will use:
//! HfDownloader::get(model_id). Everything else (download, caching, artifact
//! selection) is handled internally by the package manager.

use merix_inference::hf_downloader::HfDownloader;

#[test]
fn integration_hf_downloader_smoke_test() {
    const TEST_MODEL_ID: &str = "sentence-transformers/all-MiniLM-L6-v2";

    println!("\n=== HF DOWNLOADER HIGH-LEVEL ACQUISITION TEST ===");
    println!("Requesting model: {}", TEST_MODEL_ID);

    // This is the only call other crates should ever make
    let acquired = HfDownloader::get(TEST_MODEL_ID)
        .expect("HfDownloader::get failed");

    println!("✅ Model acquired successfully");
    println!("Local directory: {}", acquired.local_dir.display());
    println!("=============================\n");

    // Basic sanity check
    assert!(acquired.local_dir.exists(), "model directory should exist");
}
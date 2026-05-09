// backend/crates/inference/src/hf_downloader/downloader.rs
//! Intelligent model acquisition system (AI Package Manager) for Merix-v2.
//!
//! This is the single high-level entry point other crates should use.
//! Callers never see file names, repo internals, or format details.
//!
//! Overall plan for this module (TODOs below describe the full roadmap):
//! 1. Simple high-level `get(model_id)` API for immediate use
//! 2. Repo inspection + manifest parsing
//! 3. Automatic format detection (GGUF, safetensors, ONNX, etc.)
//! 4. Capability detection (embedding, generation, reranker, etc.)
//! 5. Runtime compatibility mapping
//! 6. Artifact selection + dependency resolution
//! 7. Local registry + versioning
//! 8. Policy support (offline mode, quantization preferences, etc.)

use std::path::PathBuf;

use hf_hub::api::sync::Api;
use merix_core::{Config, MerixError};

/// Result of acquiring a model — ready for loading by any runtime/embedder.
pub struct AcquiredModel {
    /// Local directory containing the model files.
    pub local_dir: PathBuf,
    // TODO: Add more metadata as we implement intelligence
    // pub format: ModelFormat,
    // pub capabilities: Vec<ModelCapability>,
    // pub recommended_runtime: RuntimeBackend,
    // pub quantization: Option<String>,
}

/// Centralized model acquisition system.
pub struct HfDownloader;

impl HfDownloader {
    /// High-level API: acquire a model by ID.
    ///
    /// This is the **only** method other crates should call.
    /// All complexity is hidden behind this single call.
    pub fn get(model_id: impl AsRef<str>) -> Result<AcquiredModel, MerixError> {
        let model_id = model_id.as_ref();

        // TODO: Full repo inspection phase (fetch config.json, read model_type, architectures, tags)
        // TODO: Capability detection (is this an embedding model? LLM? reranker? vision? speech?)
        // TODO: Format detection (GGUF? safetensors? ONNX? diffusers? GPTQ? etc.)
        // TODO: Runtime compatibility check (does this model work with llama.cpp? Candle? ONNX Runtime?)
        // TODO: Artifact selection (choose correct files based on format and runtime)
        // TODO: Dependency resolution (tokenizer, adapters, generation_config, etc.)

        let model_dir = Self::ensure_model_dir(model_id)?;

        // TODO: Make this intelligent — currently downloads common files.
        //       In the future this becomes format-aware artifact selection.
        Self::acquire_artifacts(model_id, &model_dir)?;

        Ok(AcquiredModel { local_dir: model_dir })
    }

    fn ensure_model_dir(model_id: &str) -> Result<PathBuf, MerixError> {
        let model_dir = Config::model_dir().join(model_id);

        if !model_dir.exists() {
            std::fs::create_dir_all(&model_dir)
                .map_err(|e| MerixError::Inference(format!("failed to create model directory {model_id}: {e}")))?;
        }

        Ok(model_dir)
    }

    /// Acquires all required artifacts for a model.
    fn acquire_artifacts(model_id: &str, model_dir: &PathBuf) -> Result<(), MerixError> {
        // TODO: Replace this MVP implementation with full intelligent selection
        // TODO: Detect model format first (GGUF vs safetensors vs ONNX vs diffusers etc.)
        // TODO: For GGUF models: only download *.gguf + tokenizer if present
        // TODO: For Candle/safetensors models: download config + tokenizer + model.safetensors
        // TODO: Support sharded models (model-00001-of-*.safetensors)
        // TODO: Support fine-tuned models / adapters (adapter_model.safetensors + adapter_config.json)
        // TODO: Support SentenceTransformers layout (modules.json, config_sentence_transformers.json)
        // TODO: Add quantization filtering (prefer Q4_K_M, etc. when multiple GGUF files exist)

        Self::ensure_file(model_id, "config.json", model_dir)?;
        Self::ensure_file(model_id, "tokenizer.json", model_dir)?;

        // Prefer safetensors, fall back to pytorch_model.bin (Candle-style)
        if !model_dir.join("model.safetensors").exists() && !model_dir.join("pytorch_model.bin").exists() {
            Self::ensure_file(model_id, "model.safetensors", model_dir)
                .or_else(|_| Self::ensure_file(model_id, "pytorch_model.bin", model_dir))?;
        }

        Ok(())
    }

    fn ensure_file(model_id: &str, filename: &str, model_dir: &PathBuf) -> Result<(), MerixError> {
        let file_path = model_dir.join(filename);

        if !file_path.exists() {
            let api = Api::new()
                .map_err(|e| MerixError::Inference(format!("hf-hub init failed: {e}")))?;

            let repo = api.model(model_id.to_string());

            let downloaded_path = repo.get(filename)
                .map_err(|e| MerixError::Inference(format!("failed to download {filename} for model {model_id}: {e}")))?;

            std::fs::copy(&downloaded_path, &file_path)
                .map_err(|e| MerixError::Inference(format!("failed to copy downloaded file {filename}: {e}")))?;
        }

        Ok(())
    }
}
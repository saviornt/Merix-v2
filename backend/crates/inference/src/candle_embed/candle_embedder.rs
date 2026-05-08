// backend/crates/inference/src/candle_embed/candle_embedder.rs
//! Candle-backed text embedder using BERT-style models (Merix-v2) — Candle 0.10 compatible.
//!
//! Uses the exact same `Embedder` trait from the `text_embed` submodule.
//! Supports any BERT-compatible model (all-MiniLM-L6-v2, BGE, nomic-embed, etc.).
//! Models are downloaded automatically via hf-hub on first use.

use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
use hf_hub::{api::sync::Api, Repo, RepoType};
use merix_core::MerixError;
use tokenizers::Tokenizer;

use crate::text_embed::traits::Embedder;

/// Production Candle embedder (Candle 0.10).
///
/// # Model notes
/// - Defaults to `sentence-transformers/all-MiniLM-L6-v2` (fast 384-dim).
/// - First run downloads ~90 MB (cached in `~/.cache/huggingface/hub`).
/// - CPU only (CUDA/Metal support can be added later via `Device`).
pub struct CandleEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl CandleEmbedder {
    /// Create a new embedder from a Hugging Face model ID.
    ///
    /// # Arguments
    /// * `model_id` — e.g. `"sentence-transformers/all-MiniLM-L6-v2"`
    ///
    /// # Errors
    /// Returns `MerixError::Inference` on download, parsing, or loading failures.
    pub fn new(model_id: impl AsRef<str>) -> Result<Self, MerixError> {
        let model_id = model_id.as_ref();

        let api = Api::new()
            .map_err(|e| MerixError::Inference(format!("hf-hub init failed: {e}")))?;

        let repo = api.repo(Repo::with_revision(
            model_id.to_string(),
            RepoType::Model,
            "main".to_string(),
        ));

        // Load config
        let config_path = repo.get("config.json")
            .map_err(|e| MerixError::Inference(format!("failed to get config: {e}")))?;
        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| MerixError::Inference(format!("failed to read config file: {e}")))?;
        let config: BertConfig = serde_json::from_str(&config_str)
            .map_err(|e| MerixError::Inference(format!("failed to parse config: {e}")))?;

        // Load tokenizer
        let tokenizer_path = repo.get("tokenizer.json")
            .map_err(|e| MerixError::Inference(format!("failed to get tokenizer: {e}")))?;
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| MerixError::Inference(format!("failed to load tokenizer: {e}")))?;

        // Load weights (safetensors preferred — official Candle 0.10 pattern)
        let weights_path = repo.get("model.safetensors")
            .or_else(|_| repo.get("pytorch_model.bin"))
            .map_err(|e| MerixError::Inference(format!("no model weights found: {e}")))?;

        let device = Device::Cpu;
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], candle_transformers::models::bert::DTYPE, &device)
        }
        .map_err(|e| MerixError::Inference(format!("failed to load weights: {e}")))?;

        let model = BertModel::load(vb, &config)
            .map_err(|e| MerixError::Inference(format!("failed to load BertModel: {e}")))?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }
}

impl Embedder for CandleEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError> {
        // Tokenize (adds [CLS] and [SEP])
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| MerixError::Inference(format!("tokenization failed: {e}")))?;

        let input_ids = Tensor::new(encoding.get_ids(), &self.device)
            .map_err(|e| MerixError::Inference(format!("input_ids tensor failed: {e}")))?
            .unsqueeze(0)
            .map_err(|e| MerixError::Inference(format!("unsqueeze input_ids failed: {e}")))?;

        // All zeros for single-sentence (no segment B)
        let token_type_ids = input_ids.zeros_like()
            .map_err(|e| MerixError::Inference(format!("token_type_ids creation failed: {e}")))?;

        // Attention mask from tokenizer (1 = attend, 0 = pad)
        let attention_mask = Tensor::new(encoding.get_attention_mask(), &self.device)
            .map_err(|e| MerixError::Inference(format!("attention_mask tensor failed: {e}")))?
            .unsqueeze(0)
            .map_err(|e| MerixError::Inference(format!("unsqueeze attention_mask failed: {e}")))?;

        // Forward pass — exact Candle 0.10 signature
        let output = self.model.forward(
            &input_ids,
            &token_type_ids,
            Some(&attention_mask),
        )
        .map_err(|e| MerixError::Inference(format!("model forward failed: {e}")))?;

        // Mean pooling over sequence length (standard for sentence embeddings)
        let embeddings = output.mean(1)
            .map_err(|e| MerixError::Inference(format!("mean pooling failed: {e}")))?
            .squeeze(0)
            .map_err(|e| MerixError::Inference(format!("squeeze failed: {e}")))?
            .to_vec1::<f32>()
            .map_err(|e| MerixError::Inference(format!("embedding extraction failed: {e}")))?;

        // L2 normalize (standard for RAG/semantic search)
        let norm: f32 = embeddings.iter().map(|&x| x * x).sum::<f32>().sqrt();
        let normalized: Vec<f32> = if norm > 1e-9 {
            embeddings.iter().map(|&x| x / norm).collect()
        } else {
            embeddings
        };

        Ok(normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MODEL_ID: &str = "sentence-transformers/all-MiniLM-L6-v2";

    #[test]
    fn test_candle_embedder_new_and_embed() {
        let embedder = CandleEmbedder::new(TEST_MODEL_ID)
            .expect("failed to create CandleEmbedder");

        let embedding = embedder
            .embed("Merix-v2 is an agentic operating system for AI.")
            .expect("embed call failed");

        assert_eq!(embedding.len(), 384, "embedding dimension mismatch");
        assert!(!embedding.is_empty());

        let norm: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01, "embedding not properly normalized");
    }
}
// backend/crates/inference/src/embedders/candle_embed/candle_embedder.rs
//! Candle-backed text embedder using locally stored BERT-style models (Merix-v2).

use candle_core::Tensor;
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig, DTYPE};
use merix_core::{Config, MerixError};
use tokenizers::Tokenizer;

use crate::embedders::candle_embed::traits::CandleEmbedderTraits;

/// Production Candle embedder (local models only).
pub struct CandleEmbedder {
    model: BertModel,
    tokenizer: Tokenizer,
    _device: candle_core::Device,
}

impl CandleEmbedder {
    /// Create a new embedder from a local model name.
    pub fn new(model_name: impl AsRef<str>) -> Result<Self, MerixError> {
        let model_name = model_name.as_ref();

        let model_dir = Config::model_dir().join(model_name);

        let config_path = model_dir.join("config.json");
        let tokenizer_path = model_dir.join("tokenizer.json");

        let weights_path = {
            let safetensors = model_dir.join("model.safetensors");
            if safetensors.exists() {
                safetensors
            } else {
                model_dir.join("pytorch_model.bin")
            }
        };

        if !config_path.exists() || !tokenizer_path.exists() || !weights_path.exists() {
            return Err(MerixError::Inference(format!(
                "Embedding model not found: {model_name} (required files missing in {})",
                model_dir.display()
            )));
        }

        let config_str = std::fs::read_to_string(&config_path)
            .map_err(|e| MerixError::Inference(format!("failed to read config: {e}")))?;
        let config: BertConfig = serde_json::from_str(&config_str)
            .map_err(|e| MerixError::Inference(format!("failed to parse config: {e}")))?;

        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| MerixError::Inference(format!("failed to load tokenizer: {e}")))?;

        let device = candle_core::Device::Cpu;
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&[weights_path], DTYPE, &device)
        }
        .map_err(|e| MerixError::Inference(format!("failed to load weights: {e}")))?;

        let model = BertModel::load(vb, &config)
            .map_err(|e| MerixError::Inference(format!("failed to load BertModel: {e}")))?;

        Ok(Self {
            model,
            tokenizer,
            _device: device,
        })
    }
}

#[async_trait::async_trait]
impl CandleEmbedderTraits for CandleEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| MerixError::Inference(format!("tokenization failed: {e}")))?;

        let input_ids = Tensor::new(encoding.get_ids(), &self._device)
            .map_err(|e| MerixError::Inference(format!("input_ids tensor failed: {e}")))?
            .unsqueeze(0)
            .map_err(|e| MerixError::Inference(format!("unsqueeze input_ids failed: {e}")))?;

        let token_type_ids = input_ids.zeros_like()
            .map_err(|e| MerixError::Inference(format!("token_type_ids creation failed: {e}")))?;

        let attention_mask = Tensor::new(encoding.get_attention_mask(), &self._device)
            .map_err(|e| MerixError::Inference(format!("attention_mask tensor failed: {e}")))?
            .unsqueeze(0)
            .map_err(|e| MerixError::Inference(format!("unsqueeze attention_mask failed: {e}")))?;

        let output = self.model.forward(
            &input_ids,
            &token_type_ids,
            Some(&attention_mask),
        )
        .map_err(|e| MerixError::Inference(format!("model forward failed: {e}")))?;

        let embeddings = output.mean(1)
            .map_err(|e| MerixError::Inference(format!("mean pooling failed: {e}")))?
            .squeeze(0)
            .map_err(|e| MerixError::Inference(format!("squeeze failed: {e}")))?
            .to_vec1::<f32>()
            .map_err(|e| MerixError::Inference(format!("embedding extraction failed: {e}")))?;

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

    const TEST_MODEL_NAME: &str = "sentence-transformers/all-MiniLM-L6-v2";

    #[tokio::test]
    async fn test_candle_embedder_new_and_embed() {
        // Skip gracefully if the test model is not present (CI / fresh clone friendly)
        let model_dir = merix_core::Config::model_dir().join(TEST_MODEL_NAME);
        if !model_dir.exists() {
            eprintln!("Skipping CandleEmbedder test: test model not found at {}", model_dir.display());
            return;
        }

        let embedder = CandleEmbedder::new(TEST_MODEL_NAME)
            .expect("failed to create CandleEmbedder with test model");

        let embedding = embedder
            .embed("Merix-v2 is an agentic operating system for AI.")
            .await
            .expect("embed call failed");

        // all-MiniLM-L6-v2 produces 384-dimensional embeddings
        assert_eq!(embedding.len(), 384, "embedding dimension mismatch");
        assert!(!embedding.is_empty(), "embedding vector is empty");

        // Quick sanity check: vector should be normalized (L2 norm ≈ 1.0)
        let norm: f32 = embedding.iter().map(|&x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01, "embedding not properly normalized");
    }
}
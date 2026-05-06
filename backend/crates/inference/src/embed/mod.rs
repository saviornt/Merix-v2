impl Embedder for CandleEmbedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError> {
        // STEP 1: tokenize (you'll add HF tokenizer here)
        let tokens = simple_tokenize(text);

        // STEP 2: convert to tensor
        let input = Tensor::from_vec(
            tokens,
            (1, text.len()),
            &self.device,
        ).map_err(|e| MerixError::Db(e.to_string()))?;

        // STEP 3: run model (placeholder)
        let output = fake_model_forward(input)?;

        // STEP 4: pool (mean pooling typical for embeddings)
        let embedding = mean_pool(&output)?;

        Ok(embedding)
    }
}
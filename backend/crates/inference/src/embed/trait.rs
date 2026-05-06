use merix_core::MerixError;

pub trait Embedder {
    fn embed(&self, text: &str) -> Result<Vec<f32>, MerixError>;
}
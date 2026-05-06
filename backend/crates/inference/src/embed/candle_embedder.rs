use candle_core::{Device, Tensor};
use merix_core::MerixError;
use crate::embed::Embedder;

pub struct CandleEmbedder {
    device: Device,
    // model + tokenizer fields go here
}

impl CandleEmbedder {
    pub fn new() -> Result<Self, MerixError> {
        let device = Device::Cpu;

        Ok(Self {
            device,
        })
    }
}
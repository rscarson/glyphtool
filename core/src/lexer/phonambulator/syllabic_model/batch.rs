use super::data::SequenceItem;
use burn::{data::dataloader::batcher::Batcher, prelude::*};

#[derive(Clone, Debug)]
pub struct SequenceBatch<B: Backend> {
    pub targets: Vec<(Tensor<B, 2, Int>, Tensor<B, 2, Int>)>, // [1, seq_length]
}

#[derive(Clone)]
pub struct SequenceBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> SequenceBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<SequenceItem, SequenceBatch<B>> for SequenceBatcher<B> {
    fn batch(&self, items: Vec<SequenceItem>) -> SequenceBatch<B> {
        let mut targets = Vec::new(); // vec![1, seq_length]

        for item in &items {
            // Create input tensor [batch_size, seq_length]
            let seq_tensor = Tensor::<B, 1, Int>::from_ints(item.sequence.as_slice(), &self.device);
            let seq_tensor = seq_tensor.unsqueeze_dims(&[0]);

            // Create target tensor, same shape
            let target_tensor = Tensor::<B, 1, Int>::from_ints(item.label.as_slice(), &self.device);
            let target_tensor = target_tensor.unsqueeze_dims(&[0]);

            targets.push((seq_tensor, target_tensor));
        }

        SequenceBatch { targets }
    }
}

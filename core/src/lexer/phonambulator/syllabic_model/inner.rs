use burn::{
    nn::{
        conv::{Conv1d, Conv1dConfig},
        Dropout, DropoutConfig, Embedding, EmbeddingConfig, Linear, LinearConfig, PaddingConfig1d,
    },
    prelude::*,
    record::{BinBytesRecorder, FullPrecisionSettings, Recorder},
    tensor::activation::relu,
};

use crate::database::Database;
use crate::error::EtroisResult;

//
// Hyperparameters for the model
const EMBEDDING_DIMS: usize = 256;
const NUM_FILTERS: usize = 128;
const KERNEL_SIZE: usize = 3;
const DROPOUT: f64 = 0.2;

#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    embedding: Embedding<B>,
    conv1: Conv1d<B>,
    conv2: Conv1d<B>,
    fc1: Linear<B>,
    fc2: Linear<B>,
    dropout: Dropout,
}

impl<B: Backend> Model<B> {
    pub fn new(vocab_size: usize, device: &B::Device) -> Self {
        let embedding = EmbeddingConfig::new(vocab_size, EMBEDDING_DIMS).init(device);

        let padding = PaddingConfig1d::Explicit(KERNEL_SIZE / 2);
        let conv1 = Conv1dConfig::new(EMBEDDING_DIMS, NUM_FILTERS, KERNEL_SIZE)
            .with_padding(padding)
            .init(device);

        let padding = PaddingConfig1d::Explicit(KERNEL_SIZE / 2);
        let conv2 = Conv1dConfig::new(NUM_FILTERS, NUM_FILTERS, KERNEL_SIZE)
            .with_padding(padding)
            .init(device);

        let fc1 = LinearConfig::new(NUM_FILTERS, NUM_FILTERS).init(device);
        let fc2 = LinearConfig::new(NUM_FILTERS, 1).init(device);
        let dropout = DropoutConfig::new(DROPOUT).init();

        Self {
            embedding,
            conv1,
            conv2,
            fc1,
            fc2,
            dropout,
        }
    }

    pub fn save(self, db: &Database) -> EtroisResult<()> {
        let recorder = BinBytesRecorder::<FullPrecisionSettings>::new();
        let record = self.into_record();
        let bin = recorder.record(record, ())?;

        db.save_model(&bin)
    }

    pub fn load(device: &B::Device, db: &Database) -> EtroisResult<Self> {
        let bin = db.load_model()?;
        let recorder = BinBytesRecorder::<FullPrecisionSettings>::new();
        let record = recorder.load(bin, device)?;

        let model = Self::new(db.encoder().len(), device);
        let model = model.load_record(record);

        Ok(model)
    }

    pub fn forward(&self, input: Tensor<B, 2, Int>) -> Tensor<B, 2> {
        let x = self.embedding.forward(input); // [batch_size, seq_len, embedding_dim]
        let x = x.permute([0, 2, 1]); // Conv1D expects [batch, embedding_dim, seq_len]

        // Apply convolutional layers
        let mut x = relu(self.conv1.forward(x));
        x = relu(self.conv2.forward(x));

        // Back to [batch, seq_len, features]
        let x: Tensor<B, 3> = x.permute([0, 2, 1]);

        // Flatten seq_len and batch_size together so fc2 sees a 2D tensor
        let batch_size = x.dims()[0];
        let seq_len = x.dims()[1];
        let x = x.reshape([batch_size * seq_len, NUM_FILTERS]);

        // Fully connected layers
        let x = relu(self.fc1.forward(x)); // [batch_size, seq_len, 1]

        // Dropout layer
        let x = self.dropout.forward(x);

        let x = self.fc2.forward(x); // [batch_size * seq_len, 1]
        x.reshape([batch_size, seq_len]) // [batch_size, seq_len]
    }
}

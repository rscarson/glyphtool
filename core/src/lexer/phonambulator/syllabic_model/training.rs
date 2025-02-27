use burn::{
    data::dataloader::DataLoaderBuilder,
    module::AutodiffModule,
    nn::loss::BinaryCrossEntropyLossConfig,
    optim::{AdamConfig, GradientsParams, Optimizer},
    prelude::*,
    tensor::backend::AutodiffBackend,
};

use super::{batch::SequenceBatcher, data::SequenceDataset, inner::Model};
use crate::{database::Database, error::EtroisResult};

//
// Training parameters
const BATCH_SIZE: usize = 32;
const EPOCHS: usize = 50;
const LEARNING_RATE: f64 = 1e-3;
const LOADER_WORKERS: usize = 2;
const EARLY_STOP_PATIENCE: usize = 10;
const EARLY_STOP_DELTA: f32 = 0.001;

pub fn train<B: AutodiffBackend>(device: &B::Device, db: &Database) -> EtroisResult<()> {
    let dataset = SequenceDataset::new(db)?;
    let seed = rand::random::<u64>();
    B::seed(seed);

    // Split the dataset into training and validation
    let (validation, training) = dataset.split(0.2);

    // Create loss functions
    let loss_fn = BinaryCrossEntropyLossConfig::new()
        .with_logits(true)
        //   .with_pad_tokens(Some(vec![0]))
        .init::<B>(device);
    let valid_loss_fn = loss_fn.valid();

    // Create the model and optimizer
    let mut early_stopping = EarlyStopping::new(EARLY_STOP_PATIENCE, EARLY_STOP_DELTA);
    let mut model = Model::<B>::new(db.encoder().len(), device);
    let mut optimizer = AdamConfig::new().init::<B, Model<B>>();
    //    let mut lr_opt = StepLR::new(LEARNING_RATE, 10, 0.5);

    // Create the batcher
    let batcher_train = SequenceBatcher::<B>::new(device.clone());
    let batcher_valid = SequenceBatcher::<B>::new(device.clone());

    //
    // Create the dataloaders
    let loader_train = DataLoaderBuilder::new(batcher_train)
        .batch_size(BATCH_SIZE)
        .shuffle(seed)
        .num_workers(LOADER_WORKERS)
        .build(training);
    let loader_valid = DataLoaderBuilder::new(batcher_valid)
        .batch_size(BATCH_SIZE)
        .num_workers(LOADER_WORKERS)
        .build(validation);
    let train_num_items = loader_train.num_items();
    let valid_num_items = loader_valid.num_items();

    println!("Training with {train_num_items} samples, validating with {valid_num_items} samples",);
    for epoch in 0..EPOCHS {
        // Initialize the training and validation metrics at the start of each epoch
        let mut train_losses = vec![];
        let mut train_loss = 0.0;
        let mut valid_losses = vec![];
        let mut valid_loss = 0.0;

        // Training loop
        for batch in loader_train.iter() {
            for (input, target) in batch.targets {
                let output = model.forward(input);
                let loss = loss_fn.forward(output, target);
                train_loss += loss.clone().into_scalar().elem::<f32>();

                let grads = loss.backward(); //                             Gradients for the current backward pass
                let grads = GradientsParams::from_grads(grads, &model); //  Gradients linked to each parameter of the model
                model = optimizer.step(LEARNING_RATE, model, grads); //     Update the model using the optimizer
            }
        }

        // The averaged train loss per epoch
        let avg_train_loss = train_loss / train_num_items as f32;
        train_losses.push(avg_train_loss);

        // Get the model without autodiff
        let valid_model = model.valid();

        // Implement our validation loop
        for batch in loader_valid.iter() {
            for (input, target) in batch.targets {
                let output = valid_model.forward(input.inner());
                let loss = valid_loss_fn.forward(output, target.inner());
                valid_loss += loss.clone().into_scalar().elem::<f32>();
            }
        }

        // The averaged train loss per epoch
        let avg_valid_loss = valid_loss / valid_num_items as f32;
        valid_losses.push(avg_valid_loss);

        println!(
            "Epoch {}/{}, Avg Loss {:.4}, Avg Val Loss: {:.4}",
            epoch + 1,
            EPOCHS,
            avg_train_loss,
            avg_valid_loss,
        );

        if early_stopping.step(avg_valid_loss) {
            println!("Early stopping at epoch {}", epoch + 1);
            break;
        }
    }

    model.save(db)?;
    Ok(())
}

pub struct EarlyStopping {
    patience: usize,
    delta: f32,
    best_loss: Option<f32>,
    counter: usize,
}

impl EarlyStopping {
    pub fn new(patience: usize, delta: f32) -> Self {
        Self {
            patience,
            delta,
            best_loss: None,
            counter: 0,
        }
    }

    pub fn step(&mut self, loss: f32) -> bool {
        match self.best_loss {
            Some(best) if loss >= best + self.delta => {
                self.counter += 1;
                if self.counter >= self.patience {
                    return true; // Trigger early stopping
                }
            }
            _ => {
                self.best_loss = Some(loss);
                self.counter = 0; // Reset counter if the loss improves
            }
        }

        false // Continue training
    }
}

#[allow(dead_code)]
pub struct StepLR {
    lr: f64,
    step_size: usize,
    gamma: f64,
    epoch: usize,
}

#[allow(dead_code)]
impl StepLR {
    pub fn new(initial_lr: f64, step_size: usize, gamma: f64) -> Self {
        Self {
            lr: initial_lr,
            step_size,
            gamma,
            epoch: 0,
        }
    }

    pub fn step(&mut self) -> f64 {
        if self.epoch % self.step_size == 0 && self.epoch > 0 {
            self.lr *= self.gamma;
        }

        self.epoch += 1;
        self.lr
    }
}

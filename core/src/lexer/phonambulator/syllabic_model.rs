use crate::{database::Database, error::EtroisResult, glyphs::EncodingTable};
use burn::{
    backend::{wgpu::WgpuDevice, Autodiff, Wgpu},
    prelude::*,
    tensor::activation::sigmoid,
};
use data::split_with;

mod batch;
mod data;
mod inner;
mod training;

/// CNN With embeddings for syllabification of E'trois phonemes
///
/// Codename `Autophononimbus Array`
pub struct SyllabicModel {
    inner: inner::Model<Autodiff<Wgpu>>,
    device: WgpuDevice,
    encoder: EncodingTable,
}
impl SyllabicModel {
    /// Load the model from the database, if one exists
    ///
    /// # Errors
    /// Will return an error if the model fails to load
    pub fn new(db: &Database) -> EtroisResult<Self> {
        let encoder = db.encoder().clone();
        let device = WgpuDevice::default();
        let inner = inner::Model::load(&device, db)?;

        Ok(Self {
            inner,
            device,
            encoder,
        })
    }

    /// Train a new model from the database, then save it
    ///
    /// The database will store 3 versions of the model, so you can revert to a previous version
    ///
    /// # Errors
    /// Will return an error if the model fails to train or save
    pub fn train(db: &Database) -> EtroisResult<Self> {
        let device = WgpuDevice::default();
        training::train::<Autodiff<Wgpu>>(&device, db)?;
        Self::new(db)
    }

    /// Syllabify a word
    ///
    /// # Errors
    /// Can only fail if the model somehow didn't send floats as the sigmoid output? Who knows man
    pub fn syllabify(&self, word: &str) -> EtroisResult<String> {
        let phonemes = self.encoder.encode_word(word);

        // Convert the phonemes to a tensor and run the model
        let input: Tensor<_, 1, _> =
            Tensor::from_ints(phonemes.as_slice(), &self.device).to_device(&self.device);

        // Promote to rank 2 with seq len
        let input = input.unsqueeze_dims(&[0]); // [1, seq_len]
        let output = self.inner.forward(input);
        let output = sigmoid(output);

        // Use a 0.5 threshold to determine int values
        let output: Vec<f32> = output.to_data().to_vec()?;

        /* Print out probilities clipped to 3dp
        print!("[ ");
        for prob in &output {
            print!("{:.3} ", prob);
        }
        println!("]"); */

        let output: Vec<_> = output.into_iter().map(|v| (v - 0.5) >= 0.0).collect();
        let output: Vec<_> = output.into_iter().map(Into::<u8>::into).collect();

        let phonemes = split_with(&phonemes, &output, &self.encoder);
        Ok(self.encoder.decode_word(&phonemes))
    }

    /// Syllabify a list of words
    ///
    /// # Errors
    /// Can only fail if the model somehow didn't send floats as the sigmoid output
    pub fn syllabify_all(&self, words: &[String]) -> EtroisResult<Vec<String>> {
        let encoded_words: Vec<_> = words
            .iter()
            .map(|word| self.encoder.encode_word(word))
            .collect();

        // Convert to i32
        let encoded_words_i32 = encoded_words
            .iter()
            .map(|word| word.iter().map(|&v| i32::from(v)).collect())
            .collect::<Vec<_>>();

        // Pad the words to the same length
        let max_len = encoded_words_i32.iter().map(Vec::len).max().unwrap_or(0);
        let padded_words: Vec<_> = encoded_words_i32
            .iter()
            .map(|word| {
                let mut padded = word.clone();
                padded.resize(max_len, 0);
                padded
            })
            .collect();

        let input: Tensor<_, 2, _> =
            Tensor::<_, 2, Int>::from_data(padded_words.concat().as_slice(), &self.device)
                .reshape([words.len(), max_len])
                .to_device(&self.device);

        let output = self.inner.forward(input);
        let output = sigmoid(output);

        let output: Vec<f32> = output.to_data().to_vec()?;
        let output: Vec<_> = output.into_iter().map(|v| (v - 0.5) >= 0.0).collect();
        let output: Vec<_> = output.into_iter().map(Into::<u8>::into).collect();

        let syllabified_words = encoded_words
            .iter()
            .zip(output.chunks(max_len))
            .map(|(phonemes, split)| {
                let split_points = split.to_vec();
                let syllables = split_with(phonemes, &split_points, &self.encoder);
                self.encoder.decode_word(&syllables)
            })
            .collect();

        Ok(syllabified_words)
    }
}

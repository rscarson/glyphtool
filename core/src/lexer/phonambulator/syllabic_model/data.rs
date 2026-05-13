use crate::{database::Database, error::EtroisResult, glyphs::EncodingTable};
use burn::data::dataset::{Dataset, InMemDataset};

#[derive(Clone, Debug)]
pub struct SequenceItem {
    pub sequence: Vec<u8>, // Tokenized sequence
    pub label: Vec<u8>,    // Sequence labels
}

pub struct SequenceDataset {
    samples: InMemDataset<SequenceItem>,
}
impl SequenceDataset {
    pub fn new(db: &Database) -> EtroisResult<Self> {
        let data = db.all_mappings()?;
        let data = data.values();

        let samples = data
            .map(|phonemes| {
                let input = phonemes_to_input(phonemes, db);
                let output = phonemes_to_output(phonemes, db);

                SequenceItem {
                    sequence: input,
                    label: output,
                }
            })
            .collect();

        let samples = InMemDataset::new(samples);
        Ok(Self { samples })
    }

    /// Split this dataset into a training and validation set
    pub fn split(&self, ratio: f64) -> (Self, Self) {
        let n1 = (self.samples.len() as f64 * ratio.clamp(0.0, 1.0)).round() as usize;

        let mut parts = vec![vec![]];
        for (i, item) in self.samples.iter().enumerate() {
            if i >= n1 && parts.len() == 1 {
                parts.push(vec![]);
            }

            parts.last_mut().unwrap().push(item.clone());
        }

        if parts.len() < 2 {
            parts.push(vec![]);
        }

        let train = InMemDataset::new(parts.pop().unwrap());
        let valid = InMemDataset::new(parts.pop().unwrap());
        (Self { samples: valid }, Self { samples: train })
    }
}
impl Dataset<SequenceItem> for SequenceDataset {
    fn get(&self, index: usize) -> Option<SequenceItem> {
        self.samples.get(index)
    }

    fn len(&self) -> usize {
        self.samples.len()
    }
}

//
// Helper functions for converting phonemes to input/output samples
// And converting input/output samples back to phonemes for usage
//

/// Turns raw phonemes from the database into an input for the model
pub fn phonemes_to_input(phonemes: &[u8], db: &Database) -> Vec<u8> {
    let split_char = db.encoder().encode("-");
    phonemes
        .iter()
        .copied()
        .filter(|c| *c != split_char)
        .collect()
}

/// Turns raw phonemes from the database into an output sample for the model
pub fn phonemes_to_output(phonemes: &[u8], db: &Database) -> Vec<u8> {
    let split_char = db.encoder().encode("-");
    let mut can_split = None;
    let mut output = vec![];
    for c in phonemes {
        if c == &split_char {
            can_split = Some(());
        } else {
            let is_split = can_split.take().is_some();
            output.push(u8::from(is_split));
        }
    }

    output
}

/// Turns an input/output pair back into a phoneme sequence
/// Each input/output pair is a sequence of phonemes, with the output indicating weather the next phoneme is a syllable break
pub fn split_with(input: &[u8], output: &[u8], encoder: &EncodingTable) -> Vec<u8> {
    let split_char = encoder.encode("-");

    let mut phonemes = vec![];
    for (input, output) in input.iter().zip(output.iter()) {
        if *output == 1 {
            phonemes.push(split_char);
        }

        phonemes.push(*input);
    }

    phonemes
}

//! Special definition for the numeric glyphs
use super::Glyph;

/// Represents a given number
/// - Dots are 1, lines are 3, A whole row is 0
/// - Each row down is a power of 10 higher than the last
///
/// ```text
/// ███████████
/// █         █
/// █ █ █     █
/// █ █       █
/// █         █
/// █ █ █ █ █ █
/// █ █ █     █
/// █         █
/// █ █       █
/// █         █
/// ███████████
/// ```
#[derive(Clone)]
pub struct Number {
    rows: Vec<(u32, u32)>,
    value: u32,
}
impl Glyph for Number {
    fn pronounciation(&self) -> &'static str {
        ""
    }

    fn as_boxed(&self) -> Box<dyn Glyph> {
        Box::new(self.clone())
    }
}
impl Number {
    /// Create a new number glyph
    #[must_use]
    pub fn new(value: u32) -> Self {
        let mut i = value;
        let mut rows: Vec<(u32, u32)> = vec![];
        let mut divisor = 10;
        while i > 0 {
            let removed = i % divisor;
            let digit = removed / (divisor / 10);
            i -= removed;
            divisor *= 10;

            if digit == 0 {
                rows.push((0, 0));
                continue;
            }

            let ones = digit % 3;
            let threes = (digit - ones) / 3;
            rows.push((ones, threes));
        }

        Self { rows, value }
    }

    /// Get the rows of the number (ones, threes)
    #[must_use]
    pub fn rows(&self) -> &[(u32, u32)] {
        &self.rows
    }

    /// Get the value of the number
    #[must_use]
    pub fn value(&self) -> u32 {
        self.value
    }
}

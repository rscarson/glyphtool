//! Very simple representation of an 8bit one-channel bitmap

/// A simple bitmap renderer
pub trait ToBitmap {
    /// Converts the renderer to a bitmap
    fn to_bitmap(&self) -> Bitmap;
}

/// A simple bitmap
pub struct Bitmap(Vec<Vec<u8>>);
impl Bitmap {
    /// Create a new bitmap with the given dimensions
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Bitmap(vec![vec![0xFF; width]; height])
    }

    /// Create a new bitmap from a vector of rows
    #[must_use]
    pub fn from_vec(data: Vec<Vec<u8>>) -> Self {
        Bitmap(data)
    }

    /// Returns the inner bitmap data
    #[must_use]
    pub fn inner(&self) -> &Vec<Vec<u8>> {
        &self.0
    }

    /// Returns a mutable reference to the inner bitmap data
    #[must_use]
    pub fn inner_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.0
    }

    /// Returns the inner bitmap data
    #[must_use]
    pub fn into_inner(self) -> Vec<Vec<u8>> {
        self.0
    }

    /// Returns the size of the bitmap
    pub fn size(&self) -> (usize, usize) {
        let height = self.0.len();
        let width = self.0.first().map_or(0, Vec::len);
        (width, height)
    }

    /// Pastes the source bitmap onto the current bitmap at the given position
    pub fn paste(&mut self, src: &Self, x: usize, y: usize) {
        let (src_width, src_height) = src.size();
        let (dst_width, dst_height) = self.size();

        for (src_row, dst_row) in src.0.iter().zip(&mut self.0[y..y + src_height]) {
            #[cfg(debug_assertions)]
            if x + src_width > dst_width || y + src_height > dst_height {
                eprintln!("Bitmap::paste: out of bounds");
                eprintln!("{src}");
                return;
            }
            #[cfg(debug_assertions)]
            if src_row.len() != src_width {
                eprintln!("Bitmap::paste: invalid row width");
                eprintln!("{src}");
                return;
            }
            dst_row[x..x + src_width].copy_from_slice(src_row);
        }
    }

    /// Inverts the colors of the bitmap
    pub fn invert(&mut self) {
        for row in &mut self.0 {
            for pixel in row {
                *pixel = 255 - *pixel;
            }
        }
    }

    /// Checks if the bitmap is valid (all rows have the same width)
    pub fn is_valid(&self) -> bool {
        let width = self.0.first().map_or(0, Vec::len);
        self.0.iter().all(|row| row.len() == width)
    }
}
impl FromIterator<Vec<u8>> for Bitmap {
    fn from_iter<I: IntoIterator<Item = Vec<u8>>>(iter: I) -> Self {
        Bitmap(iter.into_iter().collect())
    }
}
impl std::fmt::Display for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for &pixel in row {
                let c = match pixel {
                    0..64 => '█',
                    64..128 => '▓',
                    128..192 => '▒',
                    192..255 => '░',
                    255 => ' ',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

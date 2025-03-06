//! The trait describing the rendering for each glyph
#![allow(clippy::needless_range_loop)]
use super::bitmap::Bitmap;

/// A row of rendered pixels
///
/// Is effectively Cow<[u8]>, but with Vec
#[derive(Debug, Clone)]
pub enum RenderRow {
    /// Borrowed Static data
    Static(&'static [u8]),

    /// Owned data
    Dynamic(Vec<u8>),
}
impl AsRef<[u8]> for RenderRow {
    fn as_ref(&self) -> &[u8] {
        match self {
            RenderRow::Static(s) => s,
            RenderRow::Dynamic(d) => d,
        }
    }
}
impl RenderRow {
    /// Get the width of the row
    #[allow(clippy::len_without_is_empty)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.as_ref().len()
    }

    /// Convert to owned data
    #[must_use]
    pub fn to_owned(&self) -> Vec<u8> {
        self.as_ref().to_vec()
    }
}

/// Rendering trait for individual glyphs
pub trait Renderer {
    /// The base rendering of the glyph at minimum size
    fn render_inner(&self) -> &[RenderRow];

    /// Rows that can be stretched. Stretch is evenly distributed, with extras to the first
    fn stretch_rows(&self) -> &[usize];

    /// Columns that can be stretched. Stretch is evenly distributed, with extras to the first
    fn stretch_columns(&self) -> &[usize];

    /// The smallest size the glyph can be rendered at
    fn min_size(&self) -> (usize, usize) {
        let rows = self.render_inner().len();
        let columns = self.render_inner()[0].len();
        (columns, rows)
    }

    /// If true, the height of the glyph can be adjusted to fit the line height
    fn height_fungible(&self) -> bool {
        !self.stretch_rows().is_empty()
    }

    /// Render the glyph at a given size
    fn render_glyph(&self, width: usize, height: usize) -> Bitmap {
        let min_size = self.min_size();
        let height = height.max(min_size.1);
        let width = width.max(min_size.0);

        let mut output = Vec::with_capacity(height);
        let base = self.render_inner();
        let stretch_rows = self.stretch_rows();
        let stretch_columns = self.stretch_columns();

        //
        // Build a lookup table for the stretch rows
        let total_vstretch = height - min_size.1;
        let mut vstretch = vec![1; min_size.1];
        if !stretch_rows.is_empty() {
            for i in 0..min_size.1 {
                if !stretch_rows.contains(&i) {
                    continue;
                }

                // Stretch the rows, plus any remainder to the first row
                vstretch[i] += total_vstretch / stretch_rows.len();
                if i == stretch_rows[0] {
                    vstretch[i] += total_vstretch % stretch_rows.len();
                }
            }
        }

        //
        // Same for the columns
        let total_hstretch = width - min_size.0;
        let mut hstretch = vec![1; min_size.0];
        if !stretch_columns.is_empty() {
            for i in 0..min_size.0 {
                if !stretch_columns.contains(&i) {
                    continue;
                }

                // Stretch the columns, plus any remainder to the first column
                hstretch[i] += total_hstretch / stretch_columns.len();
                if i == stretch_columns[0] {
                    hstretch[i] += total_hstretch % stretch_columns.len();
                }
            }
        }

        //
        // Build the output
        for (i, base_row) in base.iter().enumerate() {
            let mut row = Vec::with_capacity(width);
            for (j, base_col) in base_row.as_ref().iter().enumerate() {
                // Pixel value
                let value = if *base_col == 1 { 0 } else { 255 };

                // Stretch the columns
                let hstretch = hstretch[j];
                row.extend(std::iter::repeat(value).take(hstretch));
            }

            for _ in 0..vstretch[i] {
                output.push(row.clone());
            }
        }

        Bitmap::from_vec(output)
    }
}

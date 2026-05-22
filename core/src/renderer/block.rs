use super::{
    bitmap::{Bitmap, ToBitmap},
    GlyphRowRenderer,
};
use crate::lexer::collections::Text;

/// Options for rendering a block of text
#[derive(Debug, Clone, Copy)]
pub struct GlyphBlockOptions {
    /// The margin around the block, in pixels
    pub margin: usize,

    /// If true, all glyphs will be rendered with the same height, which is the height of the tallest glyph.
    pub equalize_heights: bool,

    /// If true, line stops will be included at the end of each line
    pub include_stop: bool,

    /// If true, the source text will be rendered in ascii next to each line
    pub include_translation: bool,
}

/// Renders an entire block of text, as a series of rows
pub struct GlyphBlockRenderer {
    margin: (usize, usize),
    rows: Vec<GlyphRowRenderer>,
    width: usize,
    height: usize,
}
impl GlyphBlockRenderer {
    const MAX_ASPECT_RATIO: f32 = 3.0;

    /// Create a new block renderer
    #[must_use]
    pub fn new(text: &Text, options: GlyphBlockOptions) -> Self {
        let rows: Vec<GlyphRowRenderer> = text
            .lines()
            .iter()
            .map(|line| GlyphRowRenderer::new(line, options))
            .collect();

        let mut width = 0;
        let mut height = 0;
        for row in &rows {
            let (w, h) = row.size();
            width = width.max(w);
            height += h;
        }

        let (mut xmargin, ymargin) = (options.margin, options.margin);
        let ratio = height as f32 / width as f32;
        if ratio > Self::MAX_ASPECT_RATIO {
            // So we need h/w = MAX_ASPECT_RATIO
            // So w = h / MAX_ASPECT_RATIO
            let new_w = (width as f32 / Self::MAX_ASPECT_RATIO) as usize;
            xmargin = ((width - new_w) / 2).max(xmargin);
        }

        width += 2 * xmargin;

        let spacers = 8 * (rows.len() - 1);
        height += spacers + 2 * ymargin;

        Self {
            margin: (xmargin, ymargin),
            rows,
            width,
            height,
        }
    }

    #[must_use]
    fn render_column(rows: &[GlyphRowRenderer], width: usize, height: usize) -> Bitmap {
        let mut bitmap = Bitmap::new(width, height);
        let mut y = 0;
        for (i, row) in rows.iter().enumerate() {
            if i != 0 {
                y += 5;
            }

            let (_, height) = row.size();
            let row_bitmap = row.to_bitmap();
            bitmap.paste(&row_bitmap, 0, y);
            y += height + 3;
        }

        bitmap
    }
}
impl ToBitmap for GlyphBlockRenderer {
    fn to_bitmap(&self) -> Bitmap {
        let (xmargin, ymargin) = self.margin;

        let img = Self::render_column(
            &self.rows,
            self.width - 2 * xmargin,
            self.height - 2 * ymargin,
        );
        let mut bitmap = Bitmap::new(self.width, self.height);
        bitmap.paste(&img, xmargin, ymargin);

        bitmap
    }
}

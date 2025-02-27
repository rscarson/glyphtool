use super::{
    bitmap::{Bitmap, ToBitmap},
    GlyphRowRenderer,
};
use crate::lexer::collections::Text;

/// Renders an entire block of text, as a series of rows
pub struct GlyphBlockRenderer {
    margin: u32,
    rows: Vec<GlyphRowRenderer>,
    width: u32,
    height: u32,
}
impl GlyphBlockRenderer {
    /// Create a new block renderer
    pub fn new(text: &Text, margin: u32) -> Self {
        let rows: Vec<GlyphRowRenderer> = text.lines().iter().map(GlyphRowRenderer::new).collect();

        let mut width = 0;
        let mut height = 0;
        for row in &rows {
            let (w, h) = row.size();
            width = width.max(w);
            height += h;
        }

        width += 2 * margin;

        let spacers = 4 * (rows.len() as u32 - 1);
        height += spacers + 2 * margin;

        Self {
            margin,
            rows,
            width,
            height,
        }
    }
}
impl ToBitmap for GlyphBlockRenderer {
    fn to_bitmap(&self) -> Bitmap {
        #[rustfmt::skip]
        let spacer = vec![
            px!(e self.margin),px!(f self.width - 2 * self.margin), px!(e self.margin), px!(nl),
            px!(e self.margin),px!(f self.width - 2 * self.margin), px!(e self.margin),
        ];

        let mut bitmap = Bitmap::new(self.width as usize, self.height as usize);
        let mut y = self.margin;
        for (i, row) in self.rows.iter().enumerate() {
            if i != 0 {
                bitmap.paste(&spacer.to_bitmap(), 0, y as usize);
                y += 3;
            }

            let (_, height) = row.size();
            let row_bitmap = row.to_bitmap();
            bitmap.paste(&row_bitmap, self.margin as usize, y as usize);
            y += height + 1;
        }

        bitmap
    }
}

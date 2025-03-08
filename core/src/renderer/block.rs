use super::{
    bitmap::{Bitmap, ToBitmap},
    GlyphRowRenderer,
};
use crate::lexer::collections::Text;

/// Renders an entire block of text, as a series of rows
pub struct GlyphBlockRenderer {
    margin: usize,
    rows: Vec<GlyphRowRenderer>,
    width: usize,
    height: usize,
}
impl GlyphBlockRenderer {
    /// Create a new block renderer
    pub fn new(text: &Text, margin: usize) -> Self {
        let rows: Vec<GlyphRowRenderer> = text.lines().iter().map(GlyphRowRenderer::new).collect();

        let mut width = 0;
        let mut height = 0;
        for row in &rows {
            let (w, h) = row.size();
            width = width.max(w);
            height += h;
        }

        width += 2 * margin;

        let spacers = 8 * (rows.len() - 1);
        height += spacers + 2 * margin;

        Self {
            margin,
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
        let ratio = self.height as f32 / self.width as f32;
        let columns = (ratio / 5.0).ceil() as usize;
        let rows_per_column = self.rows.len() / columns;
        let mut start = 0;
        let mut cols = vec![];
        let mut total = 0;

        if ratio <= 3.0 {
            let img = Self::render_column(
                &self.rows,
                self.width - 2 * self.margin,
                self.height - 2 * self.margin,
            );
            let mut bitmap = Bitmap::new(self.width, self.height);
            bitmap.paste(&img, self.margin, self.margin);
            return bitmap;
        }

        for _ in 0..columns {
            let mut end = start + rows_per_column;
            if end > self.rows.len() {
                end = self.rows.len();
            }

            cols.push(&self.rows[start..end]);
            start = end;
            total += end - start;
        }
        if total < self.rows.len() {
            cols.push(&self.rows[start..]);
        }
        if cols[cols.len() - 1].is_empty() {
            cols.pop();
        }

        let mut width = 0;
        let mut height = 0;
        let mut widths = vec![];
        let mut heights = vec![];
        for col in &cols {
            let w = col.iter().map(|r| r.size().0).max().unwrap_or(0);
            widths.push(w);

            let h = col.iter().map(|r| r.size().1).sum::<usize>() + 8 * (col.len() - 1);
            heights.push(h);

            height = height.max(h);
            width += w;
        }
        width += 15 * (columns - 1) + 2 * self.margin;
        height += 2 * self.margin;

        //
        // Build v-separator
        let mut sep = vec![];
        for _ in 0..(height - 2 * self.margin) {
            sep.extend([px!(e 1), px!(f 3, 192), px!(e 1), px!(nl)]);
        }
        sep.pop();

        //
        // Render final image
        let mut bitmap = Bitmap::new(width, height);
        let mut x = self.margin;
        for (i, col) in cols.iter().enumerate() {
            if i != 0 {
                bitmap.paste(&sep.to_bitmap(), x + 3, self.margin);
                x += 11;
            }

            let col_bitmap = Self::render_column(col, widths[i], heights[i]);
            bitmap.paste(&col_bitmap, x, self.margin);
            x += widths[i];
        }

        bitmap
    }
}

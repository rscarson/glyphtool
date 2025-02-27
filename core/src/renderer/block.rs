use super::{shrtstop::join_vertical, GlyphRowRenderer, Renderer};
use crate::{lexer::collections::Text, renderer::shrtstop::ShrtstopGlyph};

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
            let (w, h) = row.min_size();
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
impl Renderer for GlyphBlockRenderer {
    fn height_fungible(&self) -> bool {
        false
    }

    fn min_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn render(&self, _: u32, _: u32) -> Vec<u32> {
        let (width, _) = self.min_size();
        let spacer = vec![
            px!(e self.margin),
            px!(f width - 2 * self.margin),
            px!(e self.margin),
            px!(nl),
        ];

        let mut pieces = vec![];
        let rows: Vec<_> = self.rows.iter().map(|r| r.render(0, 0)).collect();
        for (i, row) in rows.iter().enumerate() {
            if i != 0 {
                pieces.push(&spacer);
            }

            pieces.push(row);
        }

        join_vertical(&pieces, 1)
    }
}

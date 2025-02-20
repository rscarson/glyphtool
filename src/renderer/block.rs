use super::{utilities::insert_into_bitmap, GlyphRowRenderer};
use crate::lexer::collections::Text;

pub struct GlyphBlockRenderer {
    margin: u16,
    rows: Vec<GlyphRowRenderer>,
}
impl GlyphBlockRenderer {
    pub fn new(text: &Text, margin: u16) -> Self {
        let rows = text.lines().iter().map(GlyphRowRenderer::new).collect();
        Self { margin, rows }
    }

    pub fn width(&self) -> u16 {
        self.rows
            .iter()
            .map(GlyphRowRenderer::width)
            .max()
            .unwrap_or(0)
            + 2 * self.margin
    }

    pub fn height(&self) -> u16 {
        let row_total = self.rows.iter().map(GlyphRowRenderer::height).sum::<u16>();
        let spacers = 4 * (self.rows.len() as u16 - 1);
        row_total + spacers + 2 * self.margin
    }

    pub fn render(&self) -> Vec<Vec<u8>> {
        let (w, h) = (self.width(), self.height());
        let mut out = vec![vec![0xFF; w as usize]; h as usize];
        let spacer = vec![vec![0x00; (w - 2 * self.margin) as usize]; 2];

        let mut y = self.margin;
        for (i, row) in self.rows.iter().enumerate() {
            let row_bmp = row.render();
            insert_into_bitmap(&mut out, &row_bmp, (self.margin, y));
            y += row_bmp.len() as u16 + 1;

            if i != self.rows.len() - 1 {
                insert_into_bitmap(&mut out, &spacer, (self.margin, y));
                y += 3;
            }
        }

        out
    }
}

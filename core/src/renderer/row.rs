use super::{shrtstop::join_horizontal, GlyphStackRenderer, Renderer};
use crate::{
    glyphs::{special, AsGlyphs, Glyph},
    lexer::collections::Line,
};

/// Renders a single row of glyphs, inserting word and sentence stops as needed
pub struct GlyphRowRenderer {
    stacks: Vec<GlyphStackRenderer>,
    width: u32,
    height: u32,
}
impl GlyphRowRenderer {
    /// Create a new row renderer
    pub fn new(line: &Line) -> Self {
        let mut stacks = vec![];

        for sentence in line.sentences() {
            for word in sentence.words() {
                let glyphs = word.as_glyphs();
                let items = glyphs.into_iter().map(GlyphStackRenderer::new);

                stacks.extend(items);
                stacks.push(GlyphStackRenderer::new(vec![special::WordStop.as_boxed()]));
            }
            stacks.pop(); // Remove the last word stop

            // Add a sentence stop
            stacks.push(GlyphStackRenderer::new(vec![
                special::SentenceStop.as_boxed()
            ]));
        }

        let mut width = 0;
        let mut height = 0;
        for stack in &stacks {
            let (w, h) = stack.min_size();
            width += w;
            height = height.max(h);
        }

        width += stacks.len() as u32 - 1;
        Self {
            stacks,
            width,
            height,
        }
    }
}
impl Renderer for GlyphRowRenderer {
    fn height_fungible(&self) -> bool {
        false
    }

    fn min_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn render(&self, _: u32, h: u32) -> Vec<u32> {
        let (_, h) = self.size(0, h);

        let stacks: Vec<_> = self.stacks.iter().map(|s| s.render(0, h)).collect();
        join_horizontal(&stacks, 1)
    }
}

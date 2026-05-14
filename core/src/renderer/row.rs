use super::{
    bitmap::{Bitmap, ToBitmap},
    GlyphStackRenderer,
};
use crate::{
    glyphs::{special, AsGlyphs, Glyph},
    lexer::collections::Line,
};

/// Renders a single row of glyphs, inserting word and sentence stops as needed
pub struct GlyphRowRenderer {
    stacks: Vec<GlyphStackRenderer>,
    width: usize,
    height: usize,
}
impl GlyphRowRenderer {
    /// Create a new row renderer
    pub fn new(line: &Line, equalize_heights: bool) -> Self {
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

        if equalize_heights {
            for stack in &mut stacks {
                stack.set_height(height);
            }
        }

        width += (stacks.len() * 2) - 1;
        Self {
            stacks,
            width,
            height,
        }
    }

    /// Get the size of the row
    #[must_use]
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[allow(clippy::borrowed_box)]
    fn glyphs(&self) -> Vec<&Box<dyn Glyph>> {
        self.stacks
            .iter()
            .flat_map(GlyphStackRenderer::glyphs)
            .collect()
    }
}
impl ToBitmap for GlyphRowRenderer {
    fn to_bitmap(&self) -> Bitmap {
        let mut bitmap = Bitmap::new(self.width, self.height);

        let glyphs = self.glyphs();
        if glyphs.len() == 1 && glyphs.first().unwrap().pronounciation() == "." {
            return bitmap;
        }

        let mut x = 0;
        for stack in self.stacks.iter().map(GlyphStackRenderer::to_bitmap) {
            bitmap.paste(&stack, x, 0);
            x += stack.size().0 + 2;
        }

        bitmap
    }
}

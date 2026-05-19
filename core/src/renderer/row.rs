use super::{
    bitmap::{Bitmap, ToBitmap},
    GlyphStackRenderer,
};
use crate::{
    glyphs::{
        special::{self, WordStop},
        AsGlyphs, Glyph,
    },
    lexer::collections::Line,
    renderer::render_trait::Renderer,
};

/// Renders a single row of glyphs, inserting word and sentence stops as needed
pub struct GlyphRowRenderer {
    stacks: Vec<GlyphStackRenderer>,
    width: usize,
    height: usize,
}
impl GlyphRowRenderer {
    /// Create a new row renderer
    #[must_use]
    pub fn new(line: &Line, equalize_heights: bool, include_stop: bool) -> Self {
        let mut stacks = vec![];

        println!("Processing line: {line}");

        for sentence in line.sentences() {
            let words = sentence.words();
            for word in words {
                let glyphs = word.as_glyphs();
                let items = glyphs
                    .into_iter()
                    .map(|g| GlyphStackRenderer::new(g, equalize_heights));

                stacks.extend(items);
                stacks.push(GlyphStackRenderer::new(
                    vec![special::WordStop.as_boxed()],
                    equalize_heights,
                ));
            }
            if !words.is_empty() {
                stacks.pop(); // Remove the last word stop
            }

            // Add a sentence stop
            if include_stop {
                if let Some(last) = stacks.last() {
                    if let Some(last) = last.glyphs().last() {
                        if last.pronounciation() == "." {
                            continue;
                        }
                    }
                }

                stacks.push(GlyphStackRenderer::new(
                    vec![special::SentenceStop.as_boxed()],
                    equalize_heights,
                ));
            }
        }

        let mut width = 0;
        let mut height = 0;
        for stack in &stacks {
            let (w, h) = stack.min_size();
            width += w;
            height = height.max(h);
        }

        let min_height: usize = WordStop.min_size().1;
        let height = height.max(min_height);

        if equalize_heights {
            for stack in &mut stacks {
                stack.set_height(height);
            }
        }

        if !stacks.is_empty() {
            width += (stacks.len() * 2) - 1;
        }

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

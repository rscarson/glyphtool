use super::Renderer;
use crate::glyphs::{special::Spacer, Glyph};

/// A vertical stack of glyphs, with 1px spacing between each
/// The min-height of the stack is the sum of the min-heights of the glyphs, plus the spacing
/// The width of the stack is the max of the min-widths of the glyphs
pub struct GlyphStackRenderer {
    glyphs: Vec<Box<dyn Glyph>>,
    width: u32,
    min_height: u32,
}
impl GlyphStackRenderer {
    /// Create a new stack renderer
    #[must_use]
    pub fn new(mut glyphs: Vec<Box<dyn Glyph>>) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut has_fungible = false;

        for glyph in &glyphs {
            width = width.max(glyph.min_size().0);
            height += glyph.min_size().1;

            if glyph.height_fungible() {
                has_fungible = true;
            }
        }

        // If no fungible glyphs are present, we need to add a spacer glyph
        if !has_fungible {
            glyphs.push(Box::new(Spacer));
            height += Spacer.min_size().1;
            width = width.max(Spacer.min_size().0);
        }

        let total_spacing = glyphs.len() as u32 - 1;
        let min_height = height + total_spacing;
        Self {
            glyphs,
            width,
            min_height,
        }
    }
}
impl Renderer for GlyphStackRenderer {
    fn height_fungible(&self) -> bool {
        true
    }

    fn min_size(&self) -> (u32, u32) {
        (self.width, self.min_height)
    }

    fn render(&self, w: u32, h: u32) -> Vec<u32> {
        let (w, h) = self.size(w, h);

        // Calculate the height of each glyph
        // We increase by 1 for each fungible glyph till we reach the desired height
        let total_spacing = self.glyphs.len() as u32 - 1;
        let height_without_spacers = h - total_spacing;
        let mut height_table: Vec<_> = self.glyphs.iter().rev().map(|g| g.min_size().1).collect();
        let mut total_height = height_table.iter().sum::<u32>();
        while total_height < height_without_spacers {
            for (i, g) in self.glyphs.iter().rev().enumerate() {
                if g.height_fungible() {
                    height_table[i] += 1;
                    total_height += 1;

                    if total_height == height_without_spacers {
                        break;
                    }
                }
            }
        }

        let mut pixels = vec![];
        for (i, glyph) in self.glyphs.iter().rev().enumerate() {
            let rendered = glyph.render(w, height_table[i]);
            pixels.extend(rendered);
            if i != self.glyphs.len() - 1 {
                pixels.extend([px!(nl), px!(nl)]);
            }
        }

        pixels
    }
}

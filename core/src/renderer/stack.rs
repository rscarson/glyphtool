use super::{
    bitmap::{Bitmap, ToBitmap},
    render_trait::Renderer,
};
use crate::glyphs::{special::Spacer, Glyph};

/// A vertical stack of glyphs, with 1px spacing between each
/// The min-height of the stack is the sum of the min-heights of the glyphs, plus the spacing
/// The width of the stack is the max of the min-widths of the glyphs
pub struct GlyphStackRenderer {
    glyphs: Vec<Box<dyn Glyph>>,
    width: usize,
    min_height: usize,
    height: Option<usize>,
}
impl GlyphStackRenderer {
    /// Create a new stack renderer
    #[must_use]
    pub fn new(mut glyphs: Vec<Box<dyn Glyph>>, equalize_heights: bool) -> Self {
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
        if !has_fungible && equalize_heights {
            glyphs.push(Box::new(Spacer));
            height += Spacer.min_size().1;
            width = width.max(Spacer.min_size().0);
        }

        let total_spacing = (glyphs.len() * 2) - 1;
        let min_height = height + total_spacing;
        Self {
            glyphs,
            width,
            min_height,
            height: None,
        }
    }

    /// Get the glyphs in the stack
    #[must_use]
    pub fn glyphs(&self) -> &[Box<dyn Glyph>] {
        &self.glyphs
    }

    /// Set the actual height of the stack
    pub fn set_height(&mut self, h: usize) {
        self.height = Some(h);
    }

    /// Get the actual height of the stack
    #[must_use]
    pub fn height(&self) -> usize {
        self.height.unwrap_or(self.min_height)
    }

    /// Get the minimum size of the stack
    #[must_use]
    pub fn min_size(&self) -> (usize, usize) {
        (self.width, self.min_height)
    }
}
impl ToBitmap for GlyphStackRenderer {
    fn to_bitmap(&self) -> Bitmap {
        let h = self.height();

        // Calculate the height of each glyph
        // We increase by 1 for each fungible glyph till we reach the desired height
        let total_spacing = (self.glyphs.len() * 2) - 1;
        let height_without_spacers = h - total_spacing;
        let mut height_table: Vec<_> = self.glyphs.iter().rev().map(|g| g.min_size().1).collect();
        let mut total_height = height_table.iter().sum::<usize>();
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

        let mut bitmap = Bitmap::new(self.width, h);
        let mut y = 0;
        for (glyph, &height) in self.glyphs.iter().rev().zip(&height_table) {
            let glyph = glyph.render_glyph(self.width, height);
            bitmap.paste(&glyph, 0, y);
            y += height + 2;
        }

        bitmap
    }
}

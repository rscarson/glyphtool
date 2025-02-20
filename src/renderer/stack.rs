use super::{
    glyphs::{special, GlyphRef},
    utilities::insert_into_bitmap,
};

/// A vertical stack of glyphs, with 1px spacing between each
/// The min-height of the stack is the sum of the min-heights of the glyphs, plus the spacing
/// The width of the stack is the max of the min-widths of the glyphs
pub struct GlyphStackRenderer {
    glyphs: Vec<GlyphRef>,
}
impl GlyphStackRenderer {
    pub fn new(mut glyphs: Vec<GlyphRef>) -> Self {
        // If none of the glyphs are fungible, add a spacer
        if glyphs.iter().all(|g| !g.as_ref().height_fungible) {
            glyphs.push(GlyphRef::Owned(special::SPACER));
        }

        Self { glyphs }
    }

    /// The width at which this stack will render
    #[must_use]
    pub fn width(&self) -> u16 {
        self.glyphs
            .iter()
            .map(|g| g.as_ref().min_size.0)
            .max()
            .unwrap_or(0)
    }

    /// The smallest height at which this stack can render
    #[must_use]
    pub fn min_height(&self) -> u16 {
        let total_min = self
            .glyphs
            .iter()
            .map(|g| g.as_ref().min_size.1)
            .sum::<u16>();
        let total_spacing = self.glyphs.len() as u16 - 1;

        total_min + total_spacing
    }

    pub fn render(&self, h: u16) -> Vec<Vec<u8>> {
        let (w, h) = (self.width(), self.min_height().max(h));
        let mut out = vec![vec![0xFF; w as usize]; h as usize];

        // Calculate the height of each glyph
        // We increase by 1 for each fungible glyph till we reach the desired height
        let height_without_spacers = h - self.glyphs.len() as u16 + 1;
        let mut height_table: Vec<_> = self
            .glyphs
            .iter()
            .rev()
            .map(|g| g.as_ref().min_size.1)
            .collect();
        let mut total_height = height_table.iter().sum::<u16>();
        while total_height < height_without_spacers {
            let mut has_mutated = false; /* Sanity check! */
            for (i, g) in self.glyphs.iter().rev().enumerate() {
                if g.as_ref().height_fungible {
                    height_table[i] += 1;
                    total_height += 1;
                    has_mutated = true;

                    if total_height == height_without_spacers {
                        break;
                    }
                }
            }

            if !has_mutated {
                break;
            }
        }

        // Render each glyph, placing them at the correct position
        let mut y = 0;
        for (i, glyph) in self.glyphs.iter().rev().enumerate() {
            let rendered = glyph.as_ref().render_bitmap((w, height_table[i]));
            insert_into_bitmap(&mut out, &rendered, (0, y));

            y += height_table[i] + 1;
        }

        out
    }
}

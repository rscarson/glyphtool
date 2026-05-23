use super::{
    GlyphStackRenderer,
    bitmap::{Bitmap, ToBitmap},
};
use crate::{
    glyphs::{
        AsGlyphs, Glyph, ascii, ipa,
        special::{self, WordStop},
    },
    lexer::{collections::Line, phonambulator::glyphs_to_ipa},
    renderer::{GlyphBlockOptions, render_trait::Renderer},
};

/// Renders a single row of glyphs, inserting word and sentence stops as needed
pub struct GlyphRowRenderer {
    stacks: Vec<GlyphStackRenderer>,
    source_text: Option<Vec<Box<dyn Glyph>>>,
    ipa_text: Option<Vec<Box<dyn Glyph>>>,
    width: usize,
    height: usize,
}
impl GlyphRowRenderer {
    const CENTER_SPACING: usize = 20;

    /// Create a new row renderer
    #[must_use]
    pub fn new(line: &Line, options: GlyphBlockOptions) -> Self {
        let mut stacks = vec![];

        let source_text = line.source_text();
        let source_text = if options.include_translation {
            source_text.map(ascii::encode)
        } else {
            None
        };

        let ipa_text = if options.include_translation {
            let text = line.to_string();
            let text = glyphs_to_ipa(&text);
            Some(ipa::encode(&text))
        } else {
            None
        };

        for sentence in line.sentences() {
            let words = sentence.words();
            for word in words {
                let glyphs = word.as_glyphs();

                let items = glyphs
                    .into_iter()
                    .map(|g| GlyphStackRenderer::new(g, options.equalize_heights));

                stacks.extend(items);
                stacks.push(GlyphStackRenderer::new(
                    vec![special::WordStop.as_boxed()],
                    options.equalize_heights,
                ));
            }
            if !words.is_empty() {
                stacks.pop(); // Remove the last word stop
            }

            // Add a sentence stop
            if options.include_stop {
                if let Some(last) = stacks.last() {
                    if let Some(last) = last.glyphs().last() {
                        if last.pronounciation() == "." {
                            continue;
                        }
                    }
                }

                stacks.push(GlyphStackRenderer::new(
                    vec![special::SentenceStop.as_boxed()],
                    options.equalize_heights,
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
        height = height.max(min_height);

        if options.equalize_heights {
            for stack in &mut stacks {
                stack.set_height(height);
            }
        }

        // Add spacing between stacks and the source text
        if !stacks.is_empty() {
            width += (stacks.len() * 2) - 1;
            let mut raw_width = width;
            let mut src_height = 0;
            let mut ipa_height = 0;

            if let Some(source_text) = &source_text {
                width += Self::CENTER_SPACING;
                for glyph in source_text {
                    let (w, h) = glyph.min_size();
                    width += w + 2;
                    height = height.max(h);
                    src_height = src_height.max(h);
                }
            }
            if let Some(ipa_text) = &ipa_text {
                raw_width += Self::CENTER_SPACING;
                for glyph in ipa_text {
                    let (w, h) = glyph.min_size();
                    raw_width += w + 2;
                    height = height.max(h);
                    ipa_height = ipa_height.max(h);
                }
            }

            width = width.max(raw_width);
            height = height.max(src_height + ipa_height);
            if ipa_text.is_some() && source_text.is_some() {
                height += 2; // Add spacing between source and IPA text
            }
        }

        Self {
            stacks,
            source_text,
            ipa_text,
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

        x += Self::CENTER_SPACING;
        let end_x = x;
        let mut h_offset = 2;

        if let Some(source_text) = &self.source_text {
            for glyph in source_text {
                let g_bitmap = glyph.render_glyph(0, 0);
                bitmap.paste(&g_bitmap, x, 0);
                x += g_bitmap.size().0 + 2;
                h_offset = h_offset.max(g_bitmap.size().1 + 2);
            }
        }

        x = end_x;
        if let Some(ipa_text) = &self.ipa_text {
            for glyph in ipa_text {
                let g_bitmap = glyph.render_glyph(0, 0);
                bitmap.paste(&g_bitmap, x, h_offset);
                x += g_bitmap.size().0 + 2;
            }
        }

        bitmap
    }
}

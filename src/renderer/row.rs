use super::{
    glyphs::{special, GlyphRef},
    utilities::{glyphs_for, insert_into_bitmap},
    GlyphStackRenderer,
};
use crate::lexer::collections::Line;

pub struct GlyphRowRenderer {
    stacks: Vec<GlyphStackRenderer>,
}
impl GlyphRowRenderer {
    pub fn new(line: &Line) -> Self {
        let mut stacks = vec![];

        for sentence in line.sentences() {
            for word in sentence.words() {
                let glyphs = glyphs_for(word);
                let items = glyphs.into_iter().map(GlyphStackRenderer::new);

                stacks.extend(items);
                stacks.push(GlyphStackRenderer::new(vec![GlyphRef::Borrowed(
                    &special::WORD_STOP,
                )]));
            }
            stacks.pop(); // Remove the last word stop

            stacks.push(GlyphStackRenderer::new(vec![GlyphRef::Borrowed(
                &special::SENTENCE_STOP,
            )]));
        }

        Self { stacks }
    }

    pub fn width(&self) -> u16 {
        self.stacks
            .iter()
            .map(GlyphStackRenderer::width)
            .sum::<u16>()
            + self.stacks.len() as u16
            - 1
    }

    pub fn height(&self) -> u16 {
        self.stacks
            .iter()
            .map(GlyphStackRenderer::min_height)
            .max()
            .unwrap_or(0)
    }

    pub fn render(&self) -> Vec<Vec<u8>> {
        let (w, h) = (self.width(), self.height());
        let mut out = vec![vec![0xFF; w as usize]; h as usize];

        let mut x = 0;
        for stack in &self.stacks {
            let stack_bmp = stack.render(h);
            insert_into_bitmap(&mut out, &stack_bmp, (x, 0));

            x += stack.width() + 1;
        }

        out
    }
}

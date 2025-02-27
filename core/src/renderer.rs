//! Module dealing with the physical appearance of the glyphs in the alphabet.
//! Contains renderers for each glyph, and for various groups of glyphs.

#[macro_use]
pub mod shrtstop;

pub mod glyphs;
pub mod utilities;

mod block;
mod row;
mod stack;

pub use block::GlyphBlockRenderer;
pub use row::GlyphRowRenderer;
use shrtstop::ShrtstopGlyph;
pub use stack::GlyphStackRenderer;

/// A trait for rendering a glyph in SHRTSTOP format
pub trait Renderer {
    /// The smallest size the glyph can be rendered at
    fn min_size(&self) -> (u32, u32);

    /// Returns the actual rendered size of the glyph, given a requested size
    fn size(&self, w: u32, h: u32) -> (u32, u32) {
        let (mw, mh) = self.min_size();
        (mw.max(w), mh.max(h))
    }

    /// If true, the height of the glyph can be adjusted to fit the line height
    fn height_fungible(&self) -> bool;

    /// Renders the glyph in a special intermediate format 'SHRTSTOP'
    ///
    /// Each byte is a row of pixels, with the most significant bit being the 'is fill' flag
    ///
    /// 0 bytes are line breaks
    fn render(&self, w: u32, h: u32) -> Vec<u32>;

    /// Renders the glyph as an ASCII string
    fn render_ascii(&self, w: u32, h: u32, invert: bool) -> String {
        let rendered = self.render(w, h);
        rendered.as_ascii(invert)
    }

    /// Renders the text as a grid of pixels
    fn render_bitmap(&self, w: u32, h: u32) -> Vec<Vec<u8>> {
        let rendered = self.render(w, h);
        rendered.as_grayscale()
    }
}

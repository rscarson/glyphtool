//! Module dealing with the physical appearance of the glyphs in the alphabet.
//! Contains renderers for each glyph, and for various groups of glyphs.

#[macro_use]
pub mod shrtstop;

pub mod bitmap;

pub mod glyphs;
pub mod utilities;

mod block;
mod row;
mod stack;

pub use block::GlyphBlockRenderer;
pub use row::GlyphRowRenderer;
pub use stack::GlyphStackRenderer;

/// A trait for rendering a glyph in SHRTSTOP format
pub trait Renderer: shrtstop::ToShrtstop {
    /// The smallest size the glyph can be rendered at
    fn min_size(&self) -> (u32, u32);

    /// Returns the actual rendered size of the glyph, given a requested size
    fn size(&self, w: u32, h: u32) -> (u32, u32) {
        let (mw, mh) = self.min_size();
        (mw.max(w), mh.max(h))
    }

    /// If true, the height of the glyph can be adjusted to fit the line height
    fn height_fungible(&self) -> bool;
}

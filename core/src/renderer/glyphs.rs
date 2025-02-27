//! Contains the individual glyph renderers for the alphabet.
//! Each glyph is defined as a macro, which is then used to create a `GlyphRenderer` struct.

/// A macro to define a glyph renderer
///
/// # Example:
/// ```ignore
/// glyph!(
///    pronounciation = "ah",
///    min_size = (3, 3),
///    height_fungible = true,
///    |w, h| {
///       let mut pixels = vec![];
///       [...]
/// });
/// ```
macro_rules! glyph {
    (
        glyph = $name:path,
        min_size = ($min_width:expr, $min_height:expr),
        height_fungible = $height_fungible:expr,

        |$w:ident, $h:ident| $render:block
    ) => {
        impl $crate::renderer::Renderer for $name {
            fn min_size(&self) -> (u32, u32) {
                ($min_width, $min_height)
            }

            fn height_fungible(&self) -> bool {
                $height_fungible
            }
        }
        impl $crate::renderer::shrtstop::ToShrtstop for $name {
            fn to_shrtstop(&self, w: u32, h: u32) -> Vec<u32> {
                let ($w, $h) = <Self as $crate::renderer::Renderer>::size(self, w, h);
                $render
            }
        }
    };
}

mod consonants;
mod numbers;
mod special;
mod vowels;

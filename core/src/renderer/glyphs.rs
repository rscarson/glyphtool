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

            fn render(&self, w: u32, h: u32) -> Vec<u32> {
                let ($w, $h) = self.size(w, h);
                $render
            }
        }
    };
}

mod consonants;
mod numbers;
mod special;
mod vowels;

#[cfg(test)]
mod test {
    use crate::glyphs::ENCODING_TABLE;
    use crate::renderer::shrtstop::ShrtstopGlyph;

    #[test]
    fn test_square() {
        let mut errors = vec![];

        for glyph in ENCODING_TABLE {
            if !glyph.render(0, 0).is_square() {
                errors.push(format!(
                    "Glyph `{}` is not square at default size!",
                    glyph.pronounciation()
                ));
            }

            let h = if glyph.height_fungible() { 100 } else { 0 };
            if !glyph.render(100, h).is_square() {
                errors.push(format!(
                    "Glyph `{}` is not square at scale!",
                    glyph.pronounciation()
                ));
            }
        }

        eprintln!("Found {} errors:", errors.len());
        for error in &errors {
            eprintln!(" - {error}");
        }
        assert_eq!(errors.len(), 0);
    }
}

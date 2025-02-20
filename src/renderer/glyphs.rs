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
        pronounciation = $pronounciation:literal,
        ipa_symbol = $ipa_symbol:literal,
        min_size = ($min_width:expr, $min_height:expr),
        height_fungible = $height_fungible:expr,

        |$w:ident, $h:ident| $render:block
    ) => {
        GlyphRenderer {
            pronounciation: $pronounciation,
            ipa_symbol: $ipa_symbol,
            min_size: ($min_width, $min_height),
            height_fungible: $height_fungible,
            #[allow(unused)]
            render: super::RenderingFunction::Static(|_, $w, $h| $render),
        }
    };
}

pub mod consonants;
pub mod numbers;
pub mod special;
pub mod vowels;

pub const STOP_SOUNDS: &[&str] = &[
    ":", // Word boundary
    ".", // Sentence boundary
    "E'", "A'", "O'", // Special stops
];

pub const GLYPH_SOUND_MAP: &[(&str, &GlyphRenderer)] = &[
    //
    // Special characters
    ("?", &special::UNKNOWN),
    ("|", &special::SPACER),
    (":", &special::WORD_STOP),
    (".", &special::SENTENCE_STOP),
    ("E'", &special::DEIFIC_YE),
    ("O'", &special::POSESSIVE_HE),
    ("A'", &special::HONOURIFIC_WE),
    //
    // Consonants
    ("m", &consonants::M),
    ("f", &consonants::F),
    ("b", &consonants::B),
    ("p", &consonants::P),
    //
    ("sh", &consonants::SH),
    ("th", &consonants::TH),
    ("r", &consonants::R),
    ("s", &consonants::S),
    ("t", &consonants::T),
    ("l", &consonants::L),
    ("ng", &consonants::NG),
    ("n", &consonants::N),
    ("s", &consonants::S),
    ("z", &consonants::Z),
    ("k", &consonants::K),
    ("d", &consonants::D),
    //
    // Vowels
    ("ah", &vowels::LOW_A),
    ("uh", &vowels::HIGH_U),
    ("uh", &vowels::HIGH_U),
    ("a", &vowels::HIGH_A),
    ("u", &vowels::LOW_U),
    ("i", &vowels::I),
    ("e", &vowels::E),
    ("o", &vowels::O),
];

/// A reference to a glyph renderer (owned or borrowed)
#[derive(Debug)]
pub enum GlyphRef {
    Borrowed(&'static GlyphRenderer),
    Owned(GlyphRenderer),
}
impl AsRef<GlyphRenderer> for GlyphRef {
    fn as_ref(&self) -> &GlyphRenderer {
        match self {
            GlyphRef::Borrowed(glyph) => glyph,
            GlyphRef::Owned(glyph) => glyph,
        }
    }
}

type DynamicRenderingFunction = Box<dyn Fn(&GlyphRenderer, u16, u16) -> Vec<u16>>;
enum RenderingFunction {
    Static(fn(&GlyphRenderer, u16, u16) -> Vec<u16>),
    Dynamic(DynamicRenderingFunction),
}

/// A description of a glyph variant
pub struct GlyphRenderer {
    pub pronounciation: &'static str,
    pub ipa_symbol: &'static str,
    pub min_size: (u16, u16),
    pub height_fungible: bool,
    render: RenderingFunction,
}
impl GlyphRenderer {
    pub fn render(&self, w: u16, h: u16) -> Vec<u16> {
        let (w, h) = (w.max(self.min_size.0), h.max(self.min_size.1));
        match &self.render {
            RenderingFunction::Static(f) => f(self, w, h),
            RenderingFunction::Dynamic(f) => f(self, w, h),
        }
    }

    /// Renders the glyph as an ASCII string
    pub fn render_ascii(&self, w: u16, h: u16) -> String {
        let rendered = self.render(w, h);
        let mut ascii = String::new();
        for pixel in rendered {
            let is_fill = pixel & MASK_ISFILL == MASK_ISFILL;
            let n = pixel & !MASK_ISFILL;

            if pixel == LINEBREAK {
                ascii.push('\n');
            } else {
                let fill = if is_fill { "█" } else { " " };
                ascii.push_str(&fill.repeat(n as usize));
            }
        }

        ascii
    }

    /// Renders the text as a grid of pixels
    pub fn render_bitmap(&self, size: (u16, u16)) -> Vec<Vec<u8>> {
        let mut output = vec![vec![0xFF; size.0 as usize]; size.1 as usize];

        let rendered = self.render(size.0, size.1);
        let (mut x, mut y) = (0, 0);
        for pixel in rendered {
            let is_fill = pixel & MASK_ISFILL == MASK_ISFILL;
            let n = pixel & !MASK_ISFILL;

            if pixel == LINEBREAK {
                x = 0;
                y += 1;
            } else {
                let fill = if is_fill { 0x00 } else { 0xFF };
                for _ in 0..n {
                    output[y as usize][x as usize] = fill;
                    x += 1;
                }
            }
        }

        output
    }
}
impl std::fmt::Debug for GlyphRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GlyphRenderer")
            .field("pronounciation", &self.pronounciation)
            .field("min_size", &self.min_size)
            .field("height_fungible", &self.height_fungible)
            .finish()
    }
}

//
// Utilities for rendering glyphs
//

/// Bitmask for the "is fill" flag in a pixel
const MASK_ISFILL: u16 = 0b1000_0000_0000_0000;

/// Value for a line break in the pixel array
const LINEBREAK: u16 = 0;

/// Encode a row of n-pixels (filled or not)
fn n_pixels(n: u16, fill: bool) -> u16 {
    n | (if fill { MASK_ISFILL } else { 0 })
}

fn filled(n: u16) -> u16 {
    n_pixels(n, true)
}

fn empty(n: u16) -> u16 {
    n_pixels(n, false)
}

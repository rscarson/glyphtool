//! Glyphs for the IPA characters, to allow a tranlation sidebar
use super::Glyph;
use super::ascii;

/// Converts an ascii string to a vector of glyphs, ignoring any characters that don't have a corresponding glyph
#[must_use]
pub fn encode(s: &str) -> Vec<Box<dyn Glyph>> {
    s.to_ascii_lowercase()
        .chars()
        .filter_map(|c| match c {
            'ɑ' => Some(Box::new(IpaScriptA) as _),
            'a' => Some(Box::new(ascii::AsciiA) as _),
            'e' => Some(Box::new(ascii::AsciiE) as _),
            'i' => Some(Box::new(ascii::AsciiI) as _),
            'o' => Some(Box::new(ascii::AsciiO) as _),
            'ʌ' => Some(Box::new(IpaWedge) as _),
            'u' => Some(Box::new(ascii::AsciiU) as _),
            'ʃ' => Some(Box::new(IpaEsh) as _),
            'ð' => Some(Box::new(IpaEth) as _),
            'r' => Some(Box::new(ascii::AsciiR) as _),
            'l' => Some(Box::new(ascii::AsciiL) as _),
            'ŋ' => Some(Box::new(IpaEng) as _),
            'n' => Some(Box::new(ascii::AsciiN) as _),
            's' => Some(Box::new(ascii::AsciiS) as _),
            't' => Some(Box::new(ascii::AsciiT) as _),
            'z' => Some(Box::new(ascii::AsciiZ) as _),
            'k' => Some(Box::new(ascii::AsciiK) as _),
            'd' => Some(Box::new(ascii::AsciiD) as _),
            'm' => Some(Box::new(ascii::AsciiM) as _),
            'f' => Some(Box::new(ascii::AsciiF) as _),
            'b' => Some(Box::new(ascii::AsciiB) as _),
            'p' => Some(Box::new(ascii::AsciiP) as _),
            'h' => Some(Box::new(ascii::AsciiH) as _),
            'w' => Some(Box::new(ascii::AsciiW) as _),
            'j' => Some(Box::new(ascii::AsciiJ) as _),
            ' ' => Some(Box::new(ascii::AsciiSpace) as _),
            '.' | '!' | '?' => Some(Box::new(ascii::AsciiPeriod) as _),
            'ˈ' | '\'' => Some(Box::new(IpaUpperStress) as _),
            '/' => Some(Box::new(IpaSlash) as _),

            '0' => Some(Box::new(ascii::Ascii0) as _),
            '1' => Some(Box::new(ascii::Ascii1) as _),
            '2' => Some(Box::new(ascii::Ascii2) as _),
            '3' => Some(Box::new(ascii::Ascii3) as _),
            '4' => Some(Box::new(ascii::Ascii4) as _),
            '5' => Some(Box::new(ascii::Ascii5) as _),
            '6' => Some(Box::new(ascii::Ascii6) as _),
            '7' => Some(Box::new(ascii::Ascii7) as _),
            '8' => Some(Box::new(ascii::Ascii8) as _),
            '9' => Some(Box::new(ascii::Ascii9) as _),

            _ => None,
        })
        .collect()
}

glyph!(IpaScriptA => "ɑ", "
    The literal `ɑ`
    ```text
     ██ █
    █  ██
    █   █
    █   █
    █   █
    █  ██
     ██ █
    ```
");
impl_renderer!(
    IpaScriptA,
    glyph = [
        [0, 1, 1, 0, 1],
        [1, 0, 0, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 1, 1],
        [0, 1, 1, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(IpaWedge => "ʌ", "
    The literal `ʌ`
    ```text
       █
      █ █
      █ █
     █   █
     █   █
     █   █
     █   █
    ```
");
impl_renderer!(
    IpaWedge,
    glyph = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(IpaEsh => "ʃ", "
    The literal `ʃ`
    ```text
       ██
      █
      █
      █
      █
      █
    ██
    ```
");
impl_renderer!(
    IpaEsh,
    glyph = [
        [0, 0, 0, 1, 1],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [1, 1, 0, 0, 0]
    ],
    vstretch = [],
    hstretch = []
);

glyph!(IpaEth => "ð", "
    The literal `ð`
    ```text
    ████
      █
     ███
    █   █
    █   █
    █   █
     ███
    ```
");
impl_renderer!(
    IpaEth,
    glyph = [
        [1, 1, 1, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(IpaEng => "ŋ", "
    The literal `ŋ`
    ```text
    █ ██
    ██  █
    █    █
    █    █
    █    █
         █
       ██
    ```
");
impl_renderer!(
    IpaEng,
    glyph = [
        [1, 0, 1, 1, 0, 0],
        [1, 1, 0, 0, 1, 0],
        [1, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0, 1],
        [0, 0, 0, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = []
);
glyph!(IpaSlash => "/", "
    The literal `/`
    ```text
        █
       █
       █
      █
      █
     █
    █
    ```
");
impl_renderer!(
    IpaSlash,
    glyph = [
        [0, 0, 0, 1],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(IpaUpperStress => "ˈ", "
    The literal `ˈ`
    ```text
    █
    █
    ```
");
impl_renderer!(
    IpaUpperStress,
    glyph = [[1], [1],],
    vstretch = [],
    hstretch = []
);

//! Glyphs for the ASCII characters, to allow a tranlation sidebar
use super::Glyph;

/// Converts an ascii string to a vector of glyphs, ignoring any characters that don't have a corresponding glyph
#[must_use]
pub fn encode(s: &str) -> Vec<Box<dyn Glyph>> {
    s.to_ascii_lowercase()
        .chars()
        .filter_map(|c| match c {
            'a' => Some(Box::new(AsciiA) as _),
            'b' => Some(Box::new(AsciiB) as _),
            'c' => Some(Box::new(AsciiC) as _),
            'd' => Some(Box::new(AsciiD) as _),
            'e' => Some(Box::new(AsciiE) as _),
            'f' => Some(Box::new(AsciiF) as _),
            'g' => Some(Box::new(AsciiG) as _),
            'h' => Some(Box::new(AsciiH) as _),
            'i' => Some(Box::new(AsciiI) as _),
            'j' => Some(Box::new(AsciiJ) as _),
            'k' => Some(Box::new(AsciiK) as _),
            'l' => Some(Box::new(AsciiL) as _),
            'm' => Some(Box::new(AsciiM) as _),
            'n' => Some(Box::new(AsciiN) as _),
            'o' => Some(Box::new(AsciiO) as _),
            'p' => Some(Box::new(AsciiP) as _),
            'q' => Some(Box::new(AsciiQ) as _),
            'r' => Some(Box::new(AsciiR) as _),
            's' => Some(Box::new(AsciiS) as _),
            't' => Some(Box::new(AsciiT) as _),
            'u' => Some(Box::new(AsciiU) as _),
            'v' => Some(Box::new(AsciiV) as _),
            'w' => Some(Box::new(AsciiW) as _),
            'x' => Some(Box::new(AsciiX) as _),
            'y' => Some(Box::new(AsciiY) as _),
            'z' => Some(Box::new(AsciiZ) as _),
            ' ' => Some(Box::new(AsciiSpace) as _),
            '.' | '!' | '?' => Some(Box::new(AsciiPeriod) as _),
            '\'' => Some(Box::new(AsciiApostrophe) as _),

            '0' => Some(Box::new(Ascii0) as _),
            '1' => Some(Box::new(Ascii1) as _),
            '2' => Some(Box::new(Ascii2) as _),
            '3' => Some(Box::new(Ascii3) as _),
            '4' => Some(Box::new(Ascii4) as _),
            '5' => Some(Box::new(Ascii5) as _),
            '6' => Some(Box::new(Ascii6) as _),
            '7' => Some(Box::new(Ascii7) as _),
            '8' => Some(Box::new(Ascii8) as _),
            '9' => Some(Box::new(Ascii9) as _),

            _ => None,
        })
        .collect()
}

glyph!(AsciiA => "a", "
    The literal `a`
    ```text
    ████
    █  █
    █  █
    ████
    █  █
    █  █
    █  █
    ```
");
impl_renderer!(
    AsciiA,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiB => "b", "
    The literal `b`
    ```text
    ████
    █  █
    █  █
    ███
    █  █
    █  █
    ████
    ```
");
impl_renderer!(
    AsciiB,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiC => "c", "
    The literal `c`
    ```text
    ████
    █  
    █  
    █
    █  
    █  
    ████
    ```
");
impl_renderer!(
    AsciiC,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiD => "d", "
    The literal `d`
    ```text
    ███
    █  █
    █  █
    █  █
    █  █
    █  █
    ███
    ```
");
impl_renderer!(
    AsciiD,
    glyph = [
        [1, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiE => "e", "
    The literal `e`
    ```text
    ████
    █  
    █  
    ███  
    █  
    █  
    ████
    ```
");
impl_renderer!(
    AsciiE,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiF => "f", "
    The literal `f`
    ```text
    ████
    █  
    █  
    ███  
    █  
    █  
    █
    ```
");
impl_renderer!(
    AsciiF,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiG => "g", "
    The literal `g`
    ```text
    ████
    █  
    █  
    █ ██
    █  █
    █  █
    ████
    ```
");
impl_renderer!(
    AsciiG,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);
glyph!(AsciiH => "h", "
    The literal `h`
    ```text
    █  █
    █  █
    █  █
    ████
    █  █
    █  █
    █  █
    ```
");
impl_renderer!(
    AsciiH,
    glyph = [
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiI => "i", "
    The literal `i`
    ```text
    ███
     █
     █
     █
     █
     █
    ███
    ```
");
impl_renderer!(
    AsciiI,
    glyph = [
        [1, 1, 1],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiJ => "j", "
    The literal `j`
    ```text
      ███
        █
        █
        █
    █   █
    █   █
      ███
    ```
");
impl_renderer!(
    AsciiJ,
    glyph = [
        [0, 1, 1, 1],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [1, 0, 1, 0],
        [1, 0, 1, 0],
        [0, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiK => "k", "
    The literal `k`
    ```text
    █  █
    █ █
    ██
    █ █
    █  █
    █  █
    █  █
    ```
");
impl_renderer!(
    AsciiK,
    glyph = [
        [1, 0, 0, 1],
        [1, 0, 1, 0],
        [1, 1, 0, 0],
        [1, 0, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiL => "l", "
    The literal `l`
    ```text
    █
    █
    █
    █
    █
    █
    ████
    ```
");
impl_renderer!(
    AsciiL,
    glyph = [
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiM => "m", "
    The literal `m`
    ```text
    █   █
    ██ ██
    █ █ █
    █   █
    █   █
    █   █
    █   █
    ```
");
impl_renderer!(
    AsciiM,
    glyph = [
        [1, 0, 0, 0, 1],
        [1, 1, 0, 1, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiN => "n", "
    The literal `n`
    ```text
    █  █
    ██ █
    █ ██
    █  █
    █  █
    █  █
    █  █
    ```
");
impl_renderer!(
    AsciiN,
    glyph = [
        [1, 0, 0, 1],
        [1, 1, 0, 1],
        [1, 0, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiO => "o", "
    The literal `o`
    ```text
    ████
    █  █
    █  █
    █  █
    █  █
    █  █
    ████
    ```
");
impl_renderer!(
    AsciiO,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiP => "p", "
    The literal `p`
    ```text
    ████
    █  █
    █  █
    ████
    █
    █
    █
    ```
");
impl_renderer!(
    AsciiP,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiQ => "q", "
    The literal `q`
    ```text
    ████
    █  █
    █  █
    ████
         █
         █
         █
    ```
");
impl_renderer!(
    AsciiQ,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiR => "r", "
    The literal `r`
    ```text
    ████
    █  █
    █  █
    ████
    █ █
    █  █
    █  █
    ```
");
impl_renderer!(
    AsciiR,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [1, 0, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiS => "s", "
    The literal `s`
    ```text
    ████
    █
    █
    ████
         █
         █
    ████
    ```
");
impl_renderer!(
    AsciiS,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiT => "t", "
    The literal `t`
    ```text
    ████
      █
      █
      █
      █
      █
      █
    ```
");
impl_renderer!(
    AsciiT,
    glyph = [
        [1, 1, 1, 1],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiU => "u", "
    The literal `u`
    ```text
    █  █
    █  █
    █  █
    █  █
    █  █
    █  █
    ████
    ```
");
impl_renderer!(
    AsciiU,
    glyph = [
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiV => "v", "
    The literal `v`
    ```text
    █   █
    █   █
    █   █
     █ █
      █
      █
      █
    ```
");
impl_renderer!(
    AsciiV,
    glyph = [
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiW => "w", "
    The literal `w`
    ```text
    █   █   █
    █   █   █
    █ █ █ █
     █   █
     █   █
     █   █
     █   █
    ```
");
impl_renderer!(
    AsciiW,
    glyph = [
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 1, 0, 1, 0, 1],
        [0, 1, 0, 0, 0, 1, 0],
        [0, 1, 0, 0, 0, 1, 0],
        [0, 1, 0, 0, 0, 1, 0],
        [0, 1, 0, 0, 0, 1, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiX => "x", "
    The literal `x`
    ```text
    █   █
     █ █
      █
     █ █
    █   █
    █   █
    █   █
    ```
");
impl_renderer!(
    AsciiX,
    glyph = [
        [1, 0, 0, 1],
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiY => "y", "
    The literal `y`
    ```text
    █   █
     █ █
      █
      █
      █
      █
      █
    ```
");
impl_renderer!(
    AsciiY,
    glyph = [
        [1, 0, 0, 1],
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiZ => "z", "
    The literal `z`
    ```text
    ████
        █
       █
      █
     █
    █
    ████
    ```
");
impl_renderer!(
    AsciiZ,
    glyph = [
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 1, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiSpace => " ", "
    A space character
    ```text
    
    
    
    
    
    
    
    ```
");
impl_renderer!(
    AsciiSpace,
    glyph = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiPeriod => ".", "
    Period, exclamation mark, and question mark
    ```text
    
    
    
    
    
    ██
    ██
    ```
");
impl_renderer!(
    AsciiPeriod,
    glyph = [[0, 0], [0, 0], [0, 0], [0, 0], [0, 0], [1, 1], [1, 1],],
    vstretch = [],
    hstretch = []
);

glyph!(AsciiApostrophe => "'", "
    The apostrophe character, used for contractions and possessives
    ```text
    █
    █
    █
    
    
    
    
    ```
    ");
impl_renderer!(
    AsciiApostrophe,
    glyph = [[1], [1], [1], [0], [0], [0], [0],],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii0 => "0", "
    The literal `0`
    ```text
    ████
    █  █
    █  █
    █  █
    █  █
    █  █
    ████
    ```
");
impl_renderer!(
    Ascii0,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii1 => "1", "
    The literal `1`
    ```text
      █
     ██
      █
      █
      █
      █
     ███
    ```
");
impl_renderer!(
    Ascii1,
    glyph = [
        [0, 1, 0],
        [1, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [0, 1, 0],
        [1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii2 => "2", "
    The literal `2`
    ```text
    ████
         █
         █
    ████
    █
    █
    ████
    ```
");
impl_renderer!(
    Ascii2,
    glyph = [
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii3 => "3", "
    The literal `3`
    ```text
    ████
         █
         █
     ███
         █
         █
    ████
    ```
");
impl_renderer!(
    Ascii3,
    glyph = [
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [0, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii4 => "4", "
    The literal `4`
    ```text
    █   █
    █   █
    █   █
    █████
         █
         █
         █
    ```
");
impl_renderer!(
    Ascii4,
    glyph = [
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii5 => "5", "
    The literal `5`
    ```text
    ████
    █
    █
    ████
         █
         █
    ████
    ```
");
impl_renderer!(
    Ascii5,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii6 => "6", "
    The literal `6`
    ```text
    ████
    █
    █
    █████
    █   █
    █   █
    ████
    ```
");
impl_renderer!(
    Ascii6,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii7 => "7", "
    The literal `7`
    ```text
    ████
         █
         █
        █
       █
      █
     █
    ```
");
impl_renderer!(
    Ascii7,
    glyph = [
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [0, 0, 1, 0],
        [0, 1, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii8 => "8", "
    The literal `8`
    ```text
    ████
    █  █
    █  █
    ████
    █  █
    █  █
    ████
    ```
");
impl_renderer!(
    Ascii8,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

glyph!(Ascii9 => "9", "
    The literal `9`
    ```text
    ████
    █  █
    █  █
    █████
         █
         █
    ████
    ```
");
impl_renderer!(
    Ascii9,
    glyph = [
        [1, 1, 1, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 1, 1, 1],
        [0, 0, 0, 1],
        [0, 0, 0, 1],
        [1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = []
);

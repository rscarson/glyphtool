//! Modifier glyphs with special meanings
//!
//! These glyphs are used to modify the meaning of the word they are attached to.
//! Also included are punctuation glyphs.
use super::Glyph;

glyph!(Deific => "E'", "
    The deific modifier for the `ye` sound, as in `yellow`
    ```text
    █  █


    ████
    █
    █
  █████
    █
    █
    ███
    █
    █
    █
    ```
");
impl_renderer!(
    Deific,
    glyph = [
        [0, 0, 1, 0, 0, 1],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 1, 1, 1, 1],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 1, 1, 1, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
    ],
    vstretch = [10],
    hstretch = []
);

glyph!(Posessive => "O'", "
    The possessive modifier for the `he` sound, as in `heather`
    ```text
        █
     █  █
        █
        █
     █  █
        █
        █
      ███
        █
        █
     ███████
        █
        █
     ████
    ```
");
impl_renderer!(
    Posessive,
    glyph = [
        [0, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0],
        [1, 1, 1, 1, 0, 0, 0],
    ],
    vstretch = [2],
    hstretch = []
);

glyph!(Honourific => "A'", "
    The honourific modifier for the `we` sound, as in `weather`
    ```text      
     █  ███
     █  █  █
     █  █  █
     █  █  █
     █  █  
      ███
       ██  █  
        █   
     █  █   
     █  █  
     █  █  
      ███  █
    ```
");
impl_renderer!(
    Honourific,
    glyph = [
        [1, 0, 0, 1, 1, 1, 0],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0],
        [0, 0, 1, 1, 0, 0, 1],
        [0, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 1],
    ],
    vstretch = [3, 8],
    hstretch = []
);

//
// Punctuation
//

glyph!(Unknown => "?", "
    Placeholder glyph for unrecognized phonemes
    ```text
    ████████
    █      █
    █  ██  █
    █████  █
    ███    █
    ███  ███
    ███  ███
    ████████
    ████████
    ███  ███
    ████████
    ```
");
impl_renderer!(
    Unknown,
    glyph = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 0, 0, 1],
        [1, 1, 1, 1, 1, 0, 0, 1],
        [1, 1, 1, 0, 0, 0, 0, 1],
        [1, 1, 1, 0, 0, 1, 1, 1],
        [1, 1, 1, 0, 0, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 0, 0, 1, 1, 1],
        [1, 1, 1, 0, 0, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ],
    vstretch = [0],
    hstretch = [0]
);

glyph!(Spacer => "|", "
    Silent spacer glyph, for height adjusting consonant syllables
    ```text
        █  █
     █  █  █  █
        █  █
        █  █
     █  █  █  █
        █  █
    ```
");
impl_renderer!(
    Spacer,
    glyph = [
        [0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
        [0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
        [0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    ],
    vstretch = [2],
    hstretch = [4]
);

glyph!(WordStop => ":", "
    The stop for a word boundary
    ```text
    █


    █
    ```
");
#[rustfmt::skip]
impl_renderer!(
  WordStop,
    glyph = [
      [0],
      [1],
      [0],
      [0],
      [1],
      [0],
    ],
    vstretch = [2],
    hstretch = []
);

glyph!(SentenceStop => ".", "
    The stop for a sentence boundary
    ```text
    █  █


    █  █
    ```
");
#[rustfmt::skip]
impl_renderer!(
  SentenceStop,
    glyph = [
        [0, 0, 0, 0],
        [1, 0, 0, 1],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [1, 0, 0, 1],
        [0, 0, 0, 0],
    ],
    vstretch = [2],
    hstretch = []
);

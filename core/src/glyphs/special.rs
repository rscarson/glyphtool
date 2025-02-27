//! Modifier glyphs with special meanings
//!
//! These glyphs are used to modify the meaning of the word they are attached to.  
//! Also included are punctuation glyphs.
use super::Glyph;

glyph!(Deific => "E'", "
    The deific modifier for the `ye` sound, as in `yellow`
    ```text
    █ █

    ███
    █
   ███
    █
    ██
    █
    █
    ```
");

glyph!(Posessive => "O'", "
    The possessive modifier for the `he` sound, as in `heather`
    ```text
       █
     █ █
       █
     █ █
       █
      ██
       █
     █████
       █
    ████
    ```
");

glyph!(Honourific => "A'", "
    The honourific modifier for the `we` sound, as in `weather`
    ```text
     █ ██
       █ █
     █ █ █
      ██
       ██
     █ █ █
     █ █
      ██ █
    ```
");

//
// Punctuation
//

glyph!(Unknown => "?", "
    Placeholder glyph for unrecognized phonemes
    ```text
    █████
    █   █
    █ █ █
    ███ █
    ██ ██
    ██ ██
    █████
    ██ ██
    █████
    ```
");

glyph!(Spacer => "|", "
    Silent spacer glyph, for height adjusting consonant syllables
    ```text
       █ █
     █ █ █ █
       █ █
     █ █ █ █
       █ █
    ```
");

glyph!(WordStop => ":", "
    The stop for a word boundary
    ```text
    █

    █
    ```
");

glyph!(SentenceStop => ".", "
    The stop for a sentence boundary
    ```text
    █ █

    █ █
    ```
");

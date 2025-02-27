//! Vowel glyphs
//!
//! Vowels generally have a high and low form:
//! - ah and a (father, apple)
//! - uh and u (butt, boot)
//! - e and i (elk, eat)
//! - o (oak)
use super::Glyph;

glyph!(A => "a", "
    The high `a` sound, as in `apple`
    ```text
    █ █
    █ █
    █
    ```
");

glyph!(AH => "ah", "
    The lower `ah` sound, as in `father`
    ```text
    █
    █ █
    █ █
    ```
");

glyph!(U => "u", "
    The low `u` sound, as in `boot`
    ```text
    █ █
    █ █
     █  █
    ```
");

glyph!(UH => "uh", "
    The high `u` sound, as in `butt`
    ```text
    █ █ █
    █ █
     █
    ```
");

glyph!(I => "i", "
    The `i` sound, as in `eat`
    ```text
     █  █
    █ █
    █ █
    ```
");

glyph!(E => "e", "
    The `e` sound, as in `elk`
    ```text
     █
    █ █
    █ █ █
    ```
");

glyph!(O => "o", "
    The `o` sound, as in `oak`
    ```text
     █
    █ █
     █
    ```
");

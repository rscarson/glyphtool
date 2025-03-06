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
    █  █
    █  █
    █
    █
    ```
");
#[rustfmt::skip]
impl_renderer!(
    A,
    glyph = [
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [1, 0, 0, 0],
        [1, 0, 0, 0],
    ],
    vstretch = [2, 1],
    hstretch = [1]
);

glyph!(AH => "ah", "
    The lower `ah` sound, as in `father`
    ```text
    █
    █
    █  █
    █  █
    ```
");
#[rustfmt::skip]
impl_renderer!(
    AH,
    glyph = [
        [1, 0, 0, 0],
        [1, 0, 0, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
    ],
    vstretch = [1, 2],
    hstretch = [1]
);

glyph!(U => "u", "
    The low `u` sound, as in `boot`
    ```text
    █  █
    █  █
    █  █
    █  █  █
     ██   █
    ```
");
#[rustfmt::skip]
impl_renderer!(
    U,
    glyph = [
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 1],
        [0, 1, 1, 0, 0, 0, 1],
    ],
    vstretch = [2, 3],
    hstretch = [1]
);

glyph!(UH => "uh", "
    The high `u` sound, as in `butt`
    ```text
    █  █  █
    █  █  █
    █  █
    █  █
     ██
    ```
");
#[rustfmt::skip]
impl_renderer!(
    UH,
    glyph = [
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [0, 1, 1, 0, 0, 0, 0],
    ],
    vstretch = [2, 1],
    hstretch = [1]
);

glyph!(I => "i", "
    The `i` sound, as in `eat`
    ```text
     ██   █
    █  █  █
    █  █ 
    █  █
    █  █
    ```
");
#[rustfmt::skip]
impl_renderer!(
    I,
    glyph = [
        [0, 1, 1, 0, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
    ],
    vstretch = [2, 1],
    hstretch = [1]
);

glyph!(E => "e", "
    The `e` sound, as in `elk`
    ```text
     ██
    █  █ 
    █  █ 
    █  █  █
    █  █  █
    ```
");
#[rustfmt::skip]
impl_renderer!(
    E,
    glyph = [
        [0, 1, 1, 0, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
    ],
    vstretch = [2, 3],
    hstretch = [1]
);

glyph!(O => "o", "
    The `o` sound, as in `oak`
    ```text
     ██
    █  █
    █  █
     ██
    ```
");
#[rustfmt::skip]
impl_renderer!(
    O,
    glyph = [
        [0, 1, 1, 0],
        [1, 0, 0, 1],
        [1, 0, 0, 1],
        [0, 1, 1, 0],
    ],
    vstretch = [1],
    hstretch = [1]
);

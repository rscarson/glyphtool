//! Consonant glyphs
//!
//! The consonant glyphs are either:
//! - Open ( lips do not touch )
//! - Closed ( lips do touch )
use super::Glyph;

//
// Closed Consonants
// [M, P, B, F]
//

glyph!(M => "m", "
    The `m` sound, as in `moo`
    ```text
    ███
       █
    ███
    ```
");
impl_renderer!(
    M,
    glyph = [
        [1, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = [0]
);

glyph!(F => "f", "
    The `f` sound, as in `foo`
    ```text
     ███
    █   
     ███
    ```
");
impl_renderer!(
    F,
    glyph = [
        [0, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = [1]
);

glyph!(B => "b", "
    The `b` sound, as in `boo`
    ```text
    █  ██
     ██  █
    ```
");
#[rustfmt::skip]
impl_renderer!(
    B,
    glyph = [
        [1, 0, 0, 1, 1, 0],
        [0, 1, 1, 0, 0, 1],
    ],
    vstretch = [],
    hstretch = [2, 3]
);

glyph!(P => "p", "
    The `p` sound, as in `part`
    ```text
     ██  █
    █  ██
    ```
");
#[rustfmt::skip]
impl_renderer!(
    P,
    glyph = [
        [0, 1, 1, 0, 0, 1],
        [1, 0, 0, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = [2, 3]
);

//
// Open Consonants
// [r l t s sh th n ng k d z]
//

glyph!(R => "r", "
    The `r` sound, as in `roo`
    ```text
    ███
    ```
");
impl_renderer!(R, glyph = [[1, 1, 1],], vstretch = [], hstretch = [0]);

glyph!(L => "l", "
    The `l` sound, as in `loo`
    ```text
    ██  ██
    ```
");
impl_renderer!(
    L,
    glyph = [[1, 1, 0, 1, 1],],
    vstretch = [],
    hstretch = [2, 1, 3]
);

glyph!(T => "t", "
    The `t` sound, as in `too`
    ```text
       ██
      █
    ██
    ```
");
#[rustfmt::skip]
impl_renderer!(
    T,
    glyph = [
        [0, 0, 0, 1, 1],
        [0, 0, 1, 0, 0],
        [1, 1, 0, 0, 0],
    ],
    vstretch = [],
    hstretch = [2, 1, 3]
);

glyph!(S => "s", "
    The `s` sound, as in `small`
    ```text
    ██  ██
      ██
      ██
    ██  ██
    ```
");
impl_renderer!(
    S,
    glyph = [
        [1, 1, 0, 0, 1, 1],
        [0, 0, 1, 1, 0, 0],
        [0, 0, 1, 1, 0, 0],
        [1, 1, 0, 0, 1, 1],
    ],
    vstretch = [],
    hstretch = [2, 1, 4]
);

glyph!(Z => "z", "
    The `z` sound, as in `zoo`
    ```text
     ██  ██
    █  ██  █
    █  ██  █
     ██  ██
    ```
");
impl_renderer!(
    Z,
    glyph = [
        [0, 1, 1, 0, 0, 1, 1, 0],
        [1, 0, 0, 1, 1, 0, 0, 1],
        [1, 0, 0, 1, 1, 0, 0, 1],
        [0, 1, 1, 0, 0, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = [2, 1, 4]
);

glyph!(SH => "sh", "
    The `sh` sound, as in `shoe`
    ```text
    ██████
     █
      █
       ███
    ```
");
impl_renderer!(
    SH,
    glyph = [
        [1, 1, 1, 1, 1, 1],
        [0, 1, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0],
        [0, 0, 0, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = [4, 3, 2]
);

glyph!(TH => "th", "
    The `th` sound, as in `the`
    ```text
       ███
      █
     █
    ██████
    ```
");
impl_renderer!(
    TH,
    glyph = [
        [0, 0, 0, 1, 1, 1],
        [0, 0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = [4, 3, 2]
);

glyph!(N => "n", "
    The `n` sound, as in `no`
    ```text
     ████
    █    █
    █    █
    ██████
    ```
");
impl_renderer!(
    N,
    glyph = [
        [0, 1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = [1]
);

glyph!(NG => "ng", "
    The `ng` sound, as in `sing`
    ```text
    ██████
    █    █
    █    █
     ████
    ```
");
impl_renderer!(
    NG,
    glyph = [
        [1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 1],
        [0, 1, 1, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = [1]
);

glyph!(K => "k", "
    The `k` sound, as in `kite`
    ```text
    ███████
    █  █  █
    █  █  █
     ██ ██
    ```
");
impl_renderer!(
    K,
    glyph = [
        [1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [0, 1, 1, 0, 1, 1, 0],
    ],
    vstretch = [],
    hstretch = [3, 2, 4]
);

glyph!(D => "d", "
    The `d` sound, as in `die`
    ```text
     ██ ██
    █  █  █
    █  █  █
    ███████
    ```
");
impl_renderer!(
    D,
    glyph = [
        [0, 1, 1, 0, 1, 1, 0],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1],
    ],
    vstretch = [],
    hstretch = [3, 2, 4]
);

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

glyph!(F => "f", "
    The `f` sound, as in `foo`
    ```text
     ███
    █   
     ███
    ```
");

glyph!(B => "b", "
    The `b` sound, as in `boo`
    ```text
    █ █
     █ █
    ```
");

glyph!(P => "p", "
    The `p` sound, as in `part`
    ```text
     █ █
    █ █
    ```
");

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

glyph!(L => "l", "
    The `l` sound, as in `loo`
    ```text
    ██ ██
    ```
");

glyph!(T => "t", "
    The `t` sound, as in `too`
    ```text
       ██
      █
    ██
    ```
");

glyph!(S => "s", "
    The `s` sound, as in `small`
    ```text
    ██ ██
      █
    ██ ██
    ```
");

glyph!(Z => "z", "
    The `z` sound, as in `zoo`
    ```text
     █ █
    █ █ █
     █ █
    ```
");

glyph!(SH => "sh", "
    The `sh` sound, as in `shoe`
    ```text
    ████
     █
      ██
    ```
");

glyph!(TH => "th", "
    The `th` sound, as in `the`
    ```text
      ██
     █
    ████
    ```
");

glyph!(N => "n", "
    The `n` sound, as in `no`
    ```text
     ███
    █   █
    █████
    ```
");

glyph!(NG => "ng", "
    The `ng` sound, as in `sing`
    ```text
    █████
    █   █
     ███
    ```
");

glyph!(K => "k", "
    The `k` sound, as in `kite`
    ```text
    █████
    █ █ █
     █ █
    ```
");

glyph!(D => "d", "
    The `d` sound, as in `die`
    ```text
     █ █
    █ █ █
    █████
    ```
");

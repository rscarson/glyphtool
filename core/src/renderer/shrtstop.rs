//! SHRTSTOP2.0 Glyph Defininition Format
//!
//! Each element is defined as an unsigned 32bit integer with the following format:
//!
//! F LLLL RRR WWWWWWWWWWWWWWWWWWWWWWWW
//! | |    |   |
//! | |    |   +-- Width
//! | |    +------ Reserved bits
//! | +----------- Luminosity level
//! +------------- Fill flag
//!
//! The fill flag is 1 if the pixel is filled, 0 if it is empty.
//! The luminosity level is a value from 0 to 15, with 0 being the darkest and 15 being the lightest. If fill is 0, this value is ignored.
//! The width is the number of pixels in the run.
//!
//! 0 bytes are line breaks.
//!
//! Each block of elements is expected to be squared - that is, each row is the same length, as given by:
//! Sum [ 0x00FFFFFF & element ]
//!
#![allow(clippy::inline_always)]
#![allow(dead_code)]

/// Mask definition for the fill flag
pub const MASK_FILL: u32 = 0x8000_0000;

/// Mask definition for the luminosity level
pub const MASK_LUM: u32 = 0x7800_0000;

/// Mask definition for the width
pub const MASK_WIDTH: u32 = 0x00FF_FFFF;

/// Linebreak element value
pub const LINEBREAK: u32 = 0;

/// Macro for defining a SHRTSTOP pixel element
///
///
/// # Examples
/// ```ignore
/// px!(nl); // Linebreak
/// px!(f 1, 5); // Filled run with a width of 1px and a luminosity of 5/255
/// px!(f 2); // Filled run with a width of 2px and a luminosity of 0/255
/// px!(e 1); // Empty run with a width of 1px
/// ```
#[macro_export]
macro_rules! px {
    (nl) => {
        $crate::renderer::shrtstop::LINEBREAK
    };
    (f $w:expr, $l:expr) => {
        $crate::renderer::shrtstop::shrtstop_pixel(true, $l, $w)
    };
    (f $w:expr) => {
        $crate::renderer::shrtstop::shrtstop_pixel(true, 0, $w)
    };
    (e $w:expr) => {
        $crate::renderer::shrtstop::shrtstop_pixel(false, 0, $w)
    };
}
pub use px;

use super::bitmap::{Bitmap, ToBitmap};

#[inline]
fn shrtstop_is_linebreak(v: u32) -> bool {
    v == LINEBREAK
}

#[inline]
fn shrtstop_fill(v: u32) -> bool {
    v & MASK_FILL == MASK_FILL
}

#[inline]
fn shrtstop_lum(v: u32) -> u8 {
    ((v & MASK_LUM) >> 27) as u8
}

#[inline]
fn shrtstop_width(v: u32) -> u32 {
    v & MASK_WIDTH
}

/// Defines a SHRTSTOP pixel element
/// Expects a luminosity level from 0 to 255
#[inline(always)]
#[must_use]
pub fn shrtstop_pixel(fill: bool, lum: u8, width: u32) -> u32 {
    (u32::from(fill) << 31) | (u32::from(lum >> 4) << 27) | width
}

/// Functionality for manipulating individual pixels in the SHRTSTOP format
pub trait ShrtstopPixel {
    /// Returns true if the pixel is a linebreak
    fn is_linebreak(&self) -> bool;

    /// Returns true if the pixel is filled
    fn filled(&self) -> bool;

    /// Returns the luminosity level of the pixel  
    fn luminosity(&self) -> u8;

    /// Returns the width of the pixel
    fn width(&self) -> u32;

    /// Returns the pixel as an ASCII character
    /// This is intended for debugging purposes
    ///
    /// -  █  - Filled + Lum(0-4)
    /// -  ▓  - Filled + Lum(4-8)
    /// -  ▒  - Filled + Lum(8-12)
    /// -  ░  - Filled + Lum(12-15)
    /// -  ' '  - Empty
    /// - \\n - Linebreak
    fn as_ascii(&self, invert: bool) -> String {
        if self.is_linebreak() {
            return "\n".to_string();
        }

        let mut lum = self.luminosity() | (u8::from(!self.filled()) * 255);
        if invert {
            lum = 255 - lum;
        }
        let c = match lum {
            0..64 => '█',
            64..128 => '▓',
            128..192 => '▒',
            192..255 => '░',
            255 => ' ',
        };

        vec![c; self.width() as usize].iter().collect()
    }

    /// Returns the pixel as a grayscale byte value
    ///
    /// Will scale luminosity from 0..15 to 0..255
    /// If the pixel is empty, returns 255
    ///
    /// Whitespace is expected to be handled externally, and will result in 255
    #[inline(always)]
    fn as_grayscale(&self) -> Vec<u8> {
        let lum = self.luminosity() | (u8::from(!self.filled()) * 255);
        vec![lum.clamp(0, 240); self.width() as usize]
    }
}

impl ShrtstopPixel for u32 {
    #[inline]
    fn is_linebreak(&self) -> bool {
        shrtstop_is_linebreak(*self)
    }

    #[inline]
    fn filled(&self) -> bool {
        shrtstop_fill(*self)
    }

    #[inline]
    fn luminosity(&self) -> u8 {
        shrtstop_lum(*self) * 17
    }

    #[inline]
    fn width(&self) -> u32 {
        shrtstop_width(*self)
    }
}

/// Functionality for converting a type to a SHRTSTOP glyph
pub trait ToShrtstop {
    /// Converts the implementing type to a SHRTSTOP glyph
    fn to_shrtstop(&self, width: u32, heigh: u32) -> Vec<u32>;
}
impl ToBitmap for Vec<u32> {
    fn to_bitmap(&self) -> Bitmap {
        let rows = self.split(ShrtstopPixel::is_linebreak);
        rows.map(|row| row.iter().flat_map(ShrtstopPixel::as_grayscale).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro() {
        #[rustfmt::skip]
        let grid = [
            px!(f 1, 5), px!(e 1), px!(nl),
            px!(f 2), px!(e 1), px!(nl),
        ];

        assert_eq!(grid[0], shrtstop_pixel(true, 5, 1));
        assert_eq!(grid[1], shrtstop_pixel(false, 0, 1));
        assert_eq!(grid[2], LINEBREAK);
        assert_eq!(grid[3], shrtstop_pixel(true, 0, 2));
        assert_eq!(grid[4], shrtstop_pixel(false, 0, 1));
        assert_eq!(grid[5], LINEBREAK);
    }

    #[test]
    fn test_pixel() {
        let px = px!(f 1, 255);
        assert!(!px.is_linebreak());
        assert!(px.filled());
        assert_eq!(px.luminosity(), 255);
        assert_eq!(px.width(), 1);
    }
}

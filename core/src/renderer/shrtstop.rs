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
        vec![lum; self.width() as usize]
    }
}

/// Functionality for manipulating glyphs in the SHRTSTOP format
pub trait ShrtstopGlyph {
    /// Returns true if the glyph is properly squared  
    /// It is an error for this to return false
    ///
    /// Warning! This function is intended ONLY for debugging purposes
    /// and is highly inefficient. Do not use in production code.
    fn is_square(&self) -> bool;

    /// Returns the width of the glyph
    fn width(&self) -> u32;

    /// Returns the height of the glyph
    /// This is the number of rows in the glyph
    fn height(&self) -> u32;

    /// Returns the glyph as an ASCII string
    fn as_ascii(&self, invert: bool) -> String;

    /// Returns the glyph as a grid of grayscale values
    fn as_grayscale(&self) -> Vec<Vec<u8>>;
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

impl ShrtstopGlyph for [u32] {
    fn height(&self) -> u32 {
        self.iter().filter(|v| v.is_linebreak()).count() as u32 + 1
    }

    fn width(&self) -> u32 {
        let mut width = 0;
        for v in self {
            if v.is_linebreak() {
                break;
            }
            width += v.width();
        }

        width
    }

    fn is_square(&self) -> bool {
        let ref_width = self.width();
        let mut width = 0;
        for v in self {
            if v.is_linebreak() {
                if width != ref_width {
                    return false;
                }
                width = 0;
            } else {
                width += v.width();
            }
        }

        true
    }

    fn as_ascii(&self, invert: bool) -> String {
        self.iter().map(|g| g.as_ascii(invert)).collect()
    }

    fn as_grayscale(&self) -> Vec<Vec<u8>> {
        let rows = self.split(ShrtstopPixel::is_linebreak);
        rows.map(|row| row.iter().flat_map(ShrtstopPixel::as_grayscale).collect())
            .collect()
    }
}
impl ShrtstopGlyph for Vec<u32> {
    fn height(&self) -> u32 {
        self.as_slice().height()
    }

    fn width(&self) -> u32 {
        self.as_slice().width()
    }

    fn is_square(&self) -> bool {
        self.as_slice().is_square()
    }

    fn as_ascii(&self, invert: bool) -> String {
        self.as_slice().as_ascii(invert)
    }

    fn as_grayscale(&self) -> Vec<Vec<u8>> {
        self.as_slice().as_grayscale()
    }
}

/// Joins a series of glyphs vertically
pub fn join_vertical(pieces: &[impl AsRef<Vec<u32>>], spacing: u32) -> Vec<u32> {
    let new_width = pieces.iter().map(|p| p.as_ref().width()).max().unwrap_or(0);
    let mut result =
        Vec::with_capacity(pieces.iter().map(|p| p.as_ref().len()).sum::<usize>() + pieces.len());

    for piece in pieces {
        let piece = piece.as_ref();
        let padding = new_width - piece.width();
        for row in piece.split(ShrtstopPixel::is_linebreak) {
            result.extend(row.iter().copied());
            if padding > 0 {
                result.push(px!(e padding));
            }
            result.push(px!(nl));
        }
        for _ in 0..spacing {
            result.extend([px!(e new_width), px!(nl)]);
        }
    }

    result.pop(); // Remove the last line break
    result
}

/// Joins a series of glyphs horizontally
pub fn join_horizontal(pieces: &[impl AsRef<Vec<u32>>], spacing: u32) -> Vec<u32> {
    let mut rows: Vec<Vec<u32>> = vec![vec![]];
    for glyph in pieces {
        let stack_rows = glyph
            .as_ref()
            .split(ShrtstopPixel::is_linebreak)
            .collect::<Vec<_>>();

        let len = stack_rows.len();
        if rows.len() < len {
            let w = rows.first().map(ShrtstopGlyph::width).unwrap_or_default();
            rows.resize(len, if w > 0 { vec![px!(e w)] } else { vec![] });
        }

        for (i, row) in stack_rows.into_iter().enumerate() {
            rows[i].extend(row);

            if spacing > 0 {
                rows[i].push(px!(e spacing));
            }
        }
    }

    let mut pixels = vec![];
    for row in rows {
        pixels.extend(row);
        pixels.push(px!(nl));
    }
    pixels.pop(); // Remove the last line break
    pixels
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro() {
        #[rustfmt::skip]
        let grid = vec![
            px!(f 1, 5), px!(e 1), px!(nl),
            px!(f 2), px!(e 1), px!(nl),
        ];

        assert!(!grid.is_square());
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn test_pixel() {
        let px = px!(f 1, 255);
        assert!(!px.is_linebreak());
        assert!(px.filled());
        assert_eq!(px.luminosity(), 255);
        assert_eq!(px.width(), 1);
    }

    #[test]
    fn test_render() {
        #[rustfmt::skip]
        let grid = [
            px!(f 1, 5), px!(e 1), px!(nl),
            px!(f 2), px!(e 1), px!(nl),
        ];

        assert_eq!(grid[0].as_grayscale(), vec![0]);
        assert_eq!(grid[1].as_grayscale(), vec![255]);

        let ascii = grid.as_ascii(false);
        assert_eq!(ascii, "█ \n██ \n");
    }

    fn test_append() {
        #[rustfmt::skip]
        let left = vec![
            px!(f 1), px!(e 1), px!(nl),
            px!(f 2)
        ];

        #[rustfmt::skip]
        let right = vec![
            px!(e 2), px!(f 2), px!(nl),
            px!(f 4), px!(nl),
            px!(f 4),
        ];

        let appended = join_horizontal(&[&left, &right], 1);
        #[rustfmt::skip]
        assert_eq!(appended, vec![
            px!(f 1), px!(e 1), px!(e 1), px!(e 2), px!(f 2), px!(nl),
            px!(f 2), px!(e 1), px!(f 2), px!(f 2), px!(nl),
            px!(e 3), px!(f 4)
        ]);

        let appended = join_vertical(&[left, right], 1);
        #[rustfmt::skip]
        assert_eq!(appended, vec![
            px!(f 1), px!(e 1), px!(e 2), px!(nl),
            px!(f 2), px!(e 2), px!(nl),
            px!(nl),
            px!(e 2), px!(f 2), px!(nl),
            px!(f 4), px!(nl),
            px!(f 4)
        ]);
    }
}

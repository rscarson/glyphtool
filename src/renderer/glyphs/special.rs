use super::*;

/// Placeholder glyph for unrecognized phonemes
///
/// A solid block
pub const UNKNOWN: GlyphRenderer = glyph!(
    pronounciation = "?",
    ipa_symbol = "",
    min_size = (2, 2),
    height_fungible = true,
    |w, h| {
        let mut pixels = Vec::with_capacity(h as usize * 2 - 1);
        for i in 0..h {
            if i != 0 {
                pixels.push(LINEBREAK);
            }
            pixels.push(filled(w));
        }

        pixels
    }
);

/// Silent spacer glyph, for height adjusting consonant syllables
/// ```text
///    █ █
///  █ █ █ █
///    █ █
///  █ █ █ █
///    █ █
/// ```
#[rustfmt::skip]
pub const SPACER: GlyphRenderer = glyph!(
    pronounciation = "|",
    ipa_symbol = "",
    min_size = (7, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];

        let center_w = if w % 2 == 0 { w-5 } else { w-6 };

        for i in 0..h {
            if i != 0 {
                pixels.push(LINEBREAK);
            }

            if i % 2 == 0 {
                pixels.extend([filled(1), empty(1), filled(1), empty(center_w), filled(1), empty(1), filled(1)]);
            } else {
                pixels.extend([empty(2), filled(1), empty(center_w), filled(1)]);
            }
        }

        pixels
    }
);

/// The stop for a word boundary
/// ```text
/// █
///
/// █
/// ```
pub const WORD_STOP: GlyphRenderer = glyph!(
    pronounciation = ":",
    ipa_symbol = "",
    min_size = (1, 5),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![LINEBREAK, filled(1), LINEBREAK];

        for _ in 0..(h - 4) {
            pixels.extend([LINEBREAK]);
        }

        pixels.extend([filled(1), LINEBREAK]);
        pixels
    }
);

/// The stop for a sentence boundary
/// ```text
/// █ █
///
/// █ █
/// ```
#[rustfmt::skip]
pub const SENTENCE_STOP: GlyphRenderer = glyph!(
    pronounciation = ".",
    ipa_symbol = "",
    min_size = (3, 5),
    height_fungible = true,
    |w, h| { 
        let mut pixels = vec![LINEBREAK, filled(1), empty(1), filled(1), LINEBREAK];

        for _ in 0..(h - 4) {
            pixels.extend([LINEBREAK]);
        }

        pixels.extend([filled(1), empty(1), filled(1), LINEBREAK]);
        pixels
    }
);

/// The deific modifier for the "ye" sound, E', as in yellow
/// ```text
///  █ █
///
///  ███
///  █
/// ███
///  █
///  ██
///  █
///  █
/// ```
#[rustfmt::skip]
pub const DEIFIC_YE: GlyphRenderer = glyph!(
    pronounciation = "E'",
    ipa_symbol = "j",
    min_size = (4, 9),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![
            //
            // On top we have the dots indicating a modifier sound
            empty(1), filled(1), empty(1), filled(1), LINEBREAK,
            LINEBREAK,
            //
            // Next a line
            empty(1), filled(w - 1), LINEBREAK,
            //
            // Continuing the vertical line to the first crossbar
            empty(1), filled(1), LINEBREAK,
            filled(3), LINEBREAK,
            //
            // The 2nd crossbar
            empty(1), filled(1), LINEBREAK,
            empty(1), filled(2),
        ];

        // Now vertical line until we reach h
        for _ in 0..(h - 7) {
            pixels.extend([LINEBREAK, empty(1), filled(1)]);
        }

        pixels
    }
);

/// The possessive modifier for the "he" sound, O', as in heather
/// ```text
///    █
///  █ █
///    █
///  █ █
///    █
///   ██
///    █
///  █████
///    █
/// ████
/// ```
#[rustfmt::skip]
pub const POSESSIVE_HE: GlyphRenderer = glyph!(
    pronounciation = "O'",
    ipa_symbol = "h",
    min_size = (6, 10),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![
            //
            // On top we have the vertical bar, then the first dot
            empty(3), filled(1), LINEBREAK,
            empty(1), filled(1), empty(1), filled(1), LINEBREAK,
        ];

        // At min size, there is one line segment here - add another for each additional height
        for _ in 0..(h - 9) {
            pixels.extend([empty(3), filled(1), LINEBREAK]);
        }

        // The next dot
        pixels.extend([empty(1), filled(1), empty(1), filled(1), LINEBREAK]);

        // The next line segment
        pixels.extend([empty(3), filled(1), LINEBREAK]);

        // The first crossbar
        pixels.extend([empty(2), filled(2), LINEBREAK]);

        // The next line segment
        pixels.extend([empty(3), filled(1), LINEBREAK]);

        // The last crossbar
        pixels.extend([empty(1), filled(5), LINEBREAK]);

        // The next line segment
        pixels.extend([empty(3), filled(1), LINEBREAK]);

        // And the closer
        pixels.push(filled(4));
        pixels
    }
);

/// The honourific modifier for the "we" sound, A', as in weather
/// ```text
///  █ ██
///    █ █
///  █ █ █
///   ██
///    ██
///  █ █ █
///  █ █
///   ██ █
/// ```
#[rustfmt::skip]
pub const HONOURIFIC_WE: GlyphRenderer = glyph!(
    pronounciation = "A'",
    ipa_symbol = "w",
    min_size = (5, 8),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![filled(1), empty(1), filled(2), LINEBREAK];

        // The top and bottom have fungible segments, lets divide the extra height between them
        let extra_top = (h - 8) / 2;
        let extra_bottom = h - 8 - extra_top;

        // Repeat the last line
        for _ in 0..=extra_top {
            pixels.extend([empty(2), filled(1), empty(1), filled(1), LINEBREAK]);
        }

        // Contains the end of the TR line and the start of the ML line
        pixels.extend([
            filled(1), empty(1), filled(1), empty(1), filled(1), LINEBREAK,
        ]);

        // Center
        pixels.extend([
            empty(1), filled(2), LINEBREAK,
            empty(2), filled(2), LINEBREAK,
        ]);

        // Contains the end of the BL line and the MR line
        pixels.extend([
            filled(1), empty(1), filled(1), empty(1), filled(1), LINEBREAK,
        ]);

        // The bottom fungible
        for _ in 0..=extra_bottom {
            pixels.extend([filled(1), empty(1), filled(1), LINEBREAK]);
        }

        // And the bottom horn
        pixels.extend([empty(1), filled(2), empty(1), filled(1), LINEBREAK]);

        pixels
    }
);

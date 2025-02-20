use super::*;

/// The high "a" sound, as in "apple"
/// ```text
/// █ █
/// █ █
/// █
/// ```
pub const HIGH_A: GlyphRenderer = glyph!(
    pronounciation = "a",
    ipa_symbol = "a",
    min_size = (3, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];

        // The symbol of a set of 2 vertical lines, the rightmost which ends just below the middle vertical point
        // Spacing is given by w-2
        for i in 0..h {
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 2, false));

            if i <= h / 2 {
                pixels.push(n_pixels(1, true));
            }
            pixels.push(LINEBREAK);
        }

        pixels
    }
);

/// The lower "ah" sound, as in "father"
/// ```text
/// █
/// █ █
/// █ █
/// ```
pub const LOW_A: GlyphRenderer = glyph!(
    pronounciation = "ah",
    ipa_symbol = "ɑ",
    min_size = (3, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];

        // The symbol of a set of 2 vertical lines, the rightmost which starts just above the middle vertical point
        // Spacing is given by w-2
        for i in 0..h {
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 2, false));

            if i > h / 2 {
                pixels.push(n_pixels(1, true));
            }
            pixels.push(LINEBREAK);
        }

        pixels
    }
);

/// The high "u" sound, as in "butt"
/// ```text
/// █ █ █
/// █ █
///  █
/// ```
pub const HIGH_U: GlyphRenderer = glyph!(
    pronounciation = "uh",
    ipa_symbol = "ʊ",
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        // Top of the mark, and both side lines
        let mut pixels = vec![
            n_pixels(1, true),
            n_pixels(w - 4, false),
            n_pixels(1, true),
            n_pixels(1, false),
            n_pixels(1, true),
        ];

        // Top half adds the right mark
        for _ in 0..((h - 2) - (h / 2)) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));

            pixels.push(n_pixels(1, false));
            pixels.push(n_pixels(1, true));
        }

        // Bottom half completes the left shape, accent mark blank
        for _ in 0..(h / 2) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));
        }

        // Finally a line of w-4 pixels, with a 1px margin
        pixels.push(LINEBREAK);
        pixels.push(n_pixels(1, false));
        pixels.push(n_pixels(w - 4, true));

        pixels
    }
);

/// The low "u" sound, as in "boot"
/// ```text
/// █ █
/// █ █
///  █  █
/// ```
pub const LOW_U: GlyphRenderer = glyph!(
    pronounciation = "u",
    ipa_symbol = "u",
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        // Both side lines
        let mut pixels = vec![n_pixels(1, true), n_pixels(w - 4, false), n_pixels(1, true)];

        // Top half completes the left shape, accent mark blank
        for _ in 0..(h / 2) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));
        }

        // Bottom half adds the right mark
        for _ in 0..((h - 2) - (h / 2)) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));

            pixels.push(n_pixels(1, false));
            pixels.push(n_pixels(1, true));
        }

        // Finally a line of w-4 pixels, with a 1px margin, and mark
        pixels.push(LINEBREAK);
        pixels.push(n_pixels(1, false));
        pixels.push(n_pixels(w - 4, true));
        pixels.push(n_pixels(2, false));
        pixels.push(n_pixels(1, true));

        pixels
    }
);

/// The "i" sound, as in "eat" or "beet"
/// ```text
///  █  █
/// █ █
/// █ █
/// ```
pub const I: GlyphRenderer = glyph!(
    pronounciation = "i",
    ipa_symbol = "i",
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        // A line of w-4 pixels, with a 1px margin, and the mark
        let mut pixels = vec![
            n_pixels(1, false),
            n_pixels(w - 4, true),
            n_pixels(2, false),
            n_pixels(1, true),
        ];

        // Top half adds the right mark
        for _ in 0..((h - 2) - (h / 2)) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));

            pixels.push(n_pixels(1, false));
            pixels.push(n_pixels(1, true));
        }

        // Bottom half completes the left shape, accent mark blank
        for _ in 0..=(h / 2) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));
        }

        pixels
    }
);

/// The "e" sound, as in "elk"
/// ```text
///  █
/// █ █
/// █ █ █
/// ```
pub const E: GlyphRenderer = glyph!(
    pronounciation = "e",
    ipa_symbol = "e",
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];

        // A line of w-4 pixels, with a 1px margin
        pixels.push(n_pixels(1, false));
        pixels.push(n_pixels(w - 4, true));

        // Top half completes the left shape, accent mark blank
        for _ in 0..(h / 2) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));
        }

        // Bottom half adds the right mark
        for _ in 0..=((h - 2) - (h / 2)) {
            pixels.push(LINEBREAK);
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 4, false));
            pixels.push(n_pixels(1, true));

            pixels.push(n_pixels(1, false));
            pixels.push(n_pixels(1, true));
        }

        pixels
    }
);

/// The "o" sound, as in "oak"
/// ```text
///  █
/// █ █
///  █
/// ```
pub const O: GlyphRenderer = glyph!(
    pronounciation = "o",
    ipa_symbol = "o",
    min_size = (3, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];

        // A line of w-2 pixels, with a 1px margin
        pixels.push(n_pixels(1, false));
        pixels.push(n_pixels(w - 2, true));
        pixels.push(LINEBREAK);

        // A pair of lines, far left and far right
        for _ in 0..(h - 2) {
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 2, false));
            pixels.push(n_pixels(1, true));
            pixels.push(LINEBREAK);
        }

        // A line of w-2 pixels, with a 1px margin
        pixels.push(n_pixels(1, false));
        pixels.push(n_pixels(w - 2, true));

        pixels
    }
);

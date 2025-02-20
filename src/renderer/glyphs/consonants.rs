use super::*;

//
// Closed Consonants
// [m f p b]
//

/// The "m" sound, as in "moo"
/// ```text
/// ███
///    █
/// ███
/// ```
#[rustfmt::skip]
pub const M: GlyphRenderer = glyph!(
    pronounciation = "m",
    ipa_symbol = "m",
    min_size = (4, 3),
    height_fungible = false,
    |w, h| {
        vec![
            filled(w - 1), LINEBREAK,
            empty(w - 1), filled(1), LINEBREAK,
            filled(w - 1),
        ]
    }
);

/// The "f" sound, as in "foo"
/// ```text
///  ███
/// █   
///  ███
/// ```
#[rustfmt::skip]
pub const F: GlyphRenderer = glyph!(
    pronounciation = "f",
    ipa_symbol = "f",
    min_size = (4, 3),
    height_fungible = false,
    |w, h| {
        vec![
            empty(1), filled(w - 1), LINEBREAK,
            filled(1), LINEBREAK,
            empty(1), filled(w - 1),
        ]
    }
);

/// The "b" sound, as in "boo"
/// ```text
/// █ █
///  █ █
/// ```
#[rustfmt::skip]
pub const B: GlyphRenderer = glyph!(
    pronounciation = "b",
    ipa_symbol = "b",
    min_size = (4, 2),
    height_fungible = false,
    |w, h| {
        let seg_w = w / 4;
        let remaining = w % 4;

        let seg1 = seg_w + if remaining > 0 { 1 } else { 0 };
        let seg2 = seg_w + if remaining > 1 { 1 } else { 0 };
        let seg3 = seg_w + if remaining > 2 { 1 } else { 0 };
        let seg4 = seg_w;

        vec![
            filled(seg1), empty(seg2), filled(seg3), LINEBREAK,
            empty(seg1), filled(seg2), empty(seg3), filled(seg4),
        ]
    }
);

/// The "p" sound, as in "part" (thought i'd say poo didn't you)
/// ```text
///  █ █
/// █ █
/// ```
#[rustfmt::skip]
pub const P: GlyphRenderer = glyph!(
    pronounciation = "p",
    ipa_symbol = "p",
    min_size = (4, 2),
    height_fungible = false,
    |w, h| {
        let seg_w = w / 4;
        let remaining = w % 4;

        let seg1 = seg_w + if remaining > 0 { 1 } else { 0 };
        let seg2 = seg_w + if remaining > 1 { 1 } else { 0 };
        let seg3 = seg_w + if remaining > 2 { 1 } else { 0 };
        let seg4 = seg_w;

        vec![
            empty(seg1), filled(seg2), empty(seg3), filled(seg4), LINEBREAK,
            filled(seg1), empty(seg2), filled(seg3),
        ]
    }
);

//
// Open Consonants
// [r l t s sh th n ng k d z]
//

/// The "r" sound, as in "roo"
/// ```text
/// ███
/// ```
pub const R: GlyphRenderer = glyph!(
    pronounciation = "r",
    ipa_symbol = "r",
    min_size = (2, 1),
    height_fungible = false,
    |w, h| { vec![filled(w)] }
);

/// The "l" sound, as in "loo"
/// ```text
/// ██ ██
/// ```
pub const L: GlyphRenderer = glyph!(
    pronounciation = "l",
    ipa_symbol = "l",
    min_size = (5, 1),
    height_fungible = false,
    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2;
        vec![filled(side_w), empty(middle_w), filled(side_w)]
    }
);

/// The "t" sound, as in "too"
/// ```text
///    ██
///   █
/// ██
/// ```
#[rustfmt::skip]
pub const T: GlyphRenderer = glyph!(
    pronounciation = "t",
    ipa_symbol = "t",
    min_size = (5, 3),
    height_fungible = false,
    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2;

        vec![
            empty(side_w + middle_w), filled(side_w), LINEBREAK,
            empty(side_w), filled(middle_w), LINEBREAK,
            filled(side_w)
        ]
    }
);

/// The "s" sound, as in "small"
/// ```text
/// ██ ██
///   █
/// ██ ██
/// ```
#[rustfmt::skip]
pub const S: GlyphRenderer = glyph!(
    pronounciation = "s",
    ipa_symbol = "s",
    min_size = (5, 3),
    height_fungible = false,
    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2;

        vec![
            filled(side_w), empty(middle_w), filled(side_w), LINEBREAK,
            empty(side_w), filled(middle_w), empty(side_w), LINEBREAK,
            filled(side_w), empty(middle_w), filled(side_w),
        ]
    }
);

/// The "z" sound, as in "zoo"
/// ```text
///  █ █
/// █ █ █
///  █ █
/// ```
#[rustfmt::skip]
pub const Z: GlyphRenderer = glyph!(
    pronounciation = "z",
    ipa_symbol = "z",
    min_size = (5, 3),
    height_fungible = false,
    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2 - 1;

        vec![
            empty(1), filled(side_w), empty(middle_w), filled(side_w), empty(1), LINEBREAK,
            filled(1), empty(side_w), filled(middle_w), empty(side_w), filled(1), LINEBREAK,
            empty(1), filled(side_w), empty(middle_w), filled(side_w), empty(1),
        ]
    }
);

/// The "sh" sound, as in "shoe"
/// ```text
/// ████
///  █
///   ██
/// ```
pub const SH: GlyphRenderer = glyph!(
    pronounciation = "sh",
    ipa_symbol = "ʃ",
    min_size = (4, 3),
    height_fungible = false,
    |w, h| {
        let mut pixels = vec![];

        // Upper row is just a line of w pixels
        pixels.extend([filled(w), LINEBREAK]);

        // Next a set of h-2 rows, with a single pixel beginning in col 2, moving to the right by 1 each row
        for i in 1..h - 1 {
            pixels.extend([empty(i), filled(1), LINEBREAK]);
        }

        // Finally, the bottom row is a line of w pixels beginning one to the right of the rightmost dot above
        pixels.extend([empty(h - 1), filled(w - h + 1)]);

        pixels
    }
);

/// The "th" sound, as in "the"
/// ```text
///   ██
///  █
/// ████
/// ```
pub const TH: GlyphRenderer = glyph!(
    pronounciation = "th",
    ipa_symbol = "ð",
    min_size = (4, 3),
    height_fungible = false,
    |w, h| {
        let mut pixels = vec![];

        // Upper row is a line of w pixels beginning one to the right of the rightmost dot above
        pixels.extend([empty(h - 1), filled(w - h + 1), LINEBREAK]);

        // Next a set of h-2 rows, with a single pixel beginning in col 2, moving to the right by 1 each row
        for i in 1..h - 1 {
            pixels.extend([empty(i), filled(1), LINEBREAK]);
        }

        // Finally, the bottom row is just a line of w pixels
        pixels.extend([filled(w)]);

        pixels
    }
);

/// The "n" sound, as in "no"
/// ```text
///  ███
/// █   █
/// █████
/// ```
#[rustfmt::skip]
pub const N: GlyphRenderer = glyph!(
    pronounciation = "n",
    ipa_symbol = "n",
    min_size = (5, 3),
    height_fungible = false,
    |w, h| {
        vec![
            empty(1), filled(w - 2),  empty(1), LINEBREAK,
            filled(1), empty(w - 2), filled(1), LINEBREAK,
            filled(w),
        ]
    }
);

/// The "ng" sound, as in "sing"
/// ```text
/// █████
/// █   █
///  ███
/// ```
#[rustfmt::skip]
pub const NG: GlyphRenderer = glyph!(
    pronounciation = "ng",
    ipa_symbol = "ŋ",
    min_size = (5, 3),
    height_fungible = false,
    |w, h| {
        vec![
            filled(w), LINEBREAK,
            filled(1), empty(w - 2), filled(1), LINEBREAK,
            empty(1), filled(w - 2), empty(1),
        ]
    }
);

/// The "k" sound, as in "kite"
/// ```text
/// █████
/// █ █ █
///  █ █
/// ```
pub const K: GlyphRenderer = glyph!(
    pronounciation = "k",
    ipa_symbol = "k",
    min_size = (5, 3),
    height_fungible = false,
    |w, h| {
        let mut pixels = vec![filled(w), LINEBREAK, filled(1)];

        // At this point we need to divide the remaining (w-2) between the 2 gaps
        let gap1 = (w - 3) / 2;
        let gap2 = w - 3 - gap1;

        // Make symmetrical
        let inter_gap = if gap2 > gap1 { 2 } else { 1 };

        pixels.extend([
            empty(gap1),
            filled(inter_gap),
            empty(gap1),
            filled(1),
            LINEBREAK,
        ]);
        pixels.extend([empty(1), filled(gap1), empty(inter_gap), filled(gap1)]);

        pixels
    }
);

/// The "d" sound, as in "die"
/// ```text
///  █ █
/// █ █ █
/// █████
/// ```
pub const D: GlyphRenderer = glyph!(
    pronounciation = "d",
    ipa_symbol = "d",
    min_size = (5, 3),
    height_fungible = false,
    |w, h| {
        let mut pixels = vec![];

        // At this point we need to divide the remaining (w-2) between the 2 gaps
        let gap1 = (w - 3) / 2;
        let gap2 = w - 3 - gap1;

        // Make symmetrical
        let inter_gap = if gap2 > gap1 { 2 } else { 1 };

        pixels.extend([
            empty(1),
            filled(gap1),
            empty(inter_gap),
            filled(gap1),
            LINEBREAK,
        ]);
        pixels.extend([
            filled(1),
            empty(gap1),
            filled(inter_gap),
            empty(gap1),
            filled(1),
            LINEBREAK,
        ]);
        pixels.extend([filled(w)]);

        pixels
    }
);

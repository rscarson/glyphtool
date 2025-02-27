#![allow(unused_variables)]
use crate::glyphs::consonants::*;

//
// Closed Consonants
// [m f p b]
//

#[rustfmt::skip]
glyph!(
    glyph = M,
    min_size = (4, 3),
    height_fungible = false,

    |w, h| {
        vec![
            px!(f w - 1), px!(e 1), px!(nl),
            px!(e w - 1), px!(f 1), px!(nl),
            px!(f w - 1), px!(e 1),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = F,
    min_size = (4, 3),
    height_fungible = false,

    |w, h| {
        vec![
            px!(e 1), px!(f w - 1), px!(nl),
            px!(f 1), px!(e w - 1), px!(nl),
            px!(e 1), px!(f w - 1),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = B,
    min_size = (4, 2),
    height_fungible = false,

    |w, h| {
        let seg_w = w / 4;
        let remaining = w % 4;

        let seg1 = seg_w + u32::from(remaining > 0);
        let seg2 = seg_w + u32::from(remaining > 1);
        let seg3 = seg_w + u32::from(remaining > 2);
        let seg4 = seg_w;

        vec![
            px!(f seg1), px!(e seg2), px!(f seg3), px!(e seg4), px!(nl),
            px!(e seg1), px!(f seg2), px!(e seg3), px!(f seg4),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = P,
    min_size = (4, 2),
    height_fungible = false,

    |w, h| {
        let seg_w = w / 4;
        let remaining = w % 4;

        let seg1 = seg_w + u32::from(remaining > 0);
        let seg2 = seg_w + u32::from(remaining > 1);
        let seg3 = seg_w + u32::from(remaining > 2);
        let seg4 = seg_w;

        vec![
            px!(e seg1), px!(f seg2), px!(e seg3), px!(f seg4), px!(nl),
            px!(f seg1), px!(e seg2), px!(f seg3), px!(e seg4),
        ]
    }
);

//
// Open Consonants
// [r l t s sh th n ng k d z]
//

#[rustfmt::skip]
glyph!(
    glyph = R,
    min_size = (3, 1),
    height_fungible = false,

    |w, h| { vec![px!(f w)] }
);

#[rustfmt::skip]
glyph!(
    glyph = L,
    min_size = (5, 1),
    height_fungible = false,

    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2;
        vec![px!(f side_w), px!(e middle_w), px!(f side_w)]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = T,
    min_size = (5, 3),
    height_fungible = false,

    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2;

        vec![
            px!(e side_w + middle_w), px!(f side_w), px!(nl),
            px!(e side_w), px!(f middle_w), px!(e side_w), px!(nl),
            px!(f side_w), px!(e middle_w + side_w),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = S,
    min_size = (5, 3),
    height_fungible = false,

    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2;

        vec![
            px!(f side_w), px!(e middle_w), px!(f side_w), px!(nl),
            px!(e side_w), px!(f middle_w), px!(e side_w), px!(nl),
            px!(f side_w), px!(e middle_w), px!(f side_w),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = Z,
    min_size = (5, 3),
    height_fungible = false,

    |w, h| {
        let middle_w = if w % 2 == 0 { 2 } else { 1 };
        let side_w = (w - middle_w) / 2 - 1;

        vec![
            px!(e 1), px!(f side_w), px!(e middle_w), px!(f side_w), px!(e 1), px!(nl),
            px!(f 1), px!(e side_w), px!(f middle_w), px!(e side_w), px!(f 1), px!(nl),
            px!(e 1), px!(f side_w), px!(e middle_w), px!(f side_w), px!(e 1),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = SH,
    min_size = (4, 3),
    height_fungible = false,

    |w, h| {
        vec![
            px!(f w), px!(nl),
            px!(e 1), px!(f 1), px!(e w - 2), px!(nl),
            px!(e 2), px!(f w - 2),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = TH,
    min_size = (4, 3),
    height_fungible = false,

    |w, h| {
        vec![
            px!(e 2), px!(f w - 2), px!(nl),
            px!(e 1), px!(f 1), px!(e w - 2), px!(nl),
            px!(f w),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = N,
    min_size = (5, 3),
    height_fungible = false,

    |w, h| {
        vec![
            px!(e 1), px!(f w - 2),  px!(e 1), px!(nl),
            px!(f 1), px!(e w - 2), px!(f 1), px!(nl),
            px!(f w),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = NG,
    min_size = (5, 3),
    height_fungible = false,

    |w, h| {
        vec![
            px!(f w), px!(nl),
            px!(f 1), px!(e w - 2), px!(f 1), px!(nl),
            px!(e 1), px!(f w - 2), px!(e 1),
        ]
    }
);

#[rustfmt::skip]
glyph!(
    glyph = K,
    min_size = (5, 3),
    height_fungible = false,

    |w, h| {
        let mut pixels = vec![px!(f w), px!(nl)];

        // At this point we need to divide the remaining (w-2) between the 2 gaps
        let gap1 = (w - 3) / 2;
        let gap2 = w - 3 - gap1;

        // Make symmetrical
        let inter_gap = if gap2 > gap1 { 2 } else { 1 };

        pixels.extend([
            px!(f 1),
            px!(e gap1),
            px!(f inter_gap),
            px!(e gap1),
            px!(f 1),
            px!(nl),
        ]);
        pixels.extend([px!(e 1), px!(f gap1), px!(e inter_gap), px!(f gap1), px!(e 1)]);

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = D,
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
            px!(e 1),
            px!(f gap1),
            px!(e inter_gap),
            px!(f gap1),
            px!(e 1),
            px!(nl),
        ]);
        pixels.extend([
            px!(f 1),
            px!(e gap1),
            px!(f inter_gap),
            px!(e gap1),
            px!(f 1),
            px!(nl),
        ]);
        pixels.extend([px!(f w)]);

        pixels
    }
);

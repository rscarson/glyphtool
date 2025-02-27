use crate::glyphs::vowels::*;

#[rustfmt::skip]
glyph!(
    glyph = A,
    min_size = (3, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];
        // The symbol of a set of 2 vertical lines, the rightmost which ends just below the middle vertical point
        // Spacing is given by w-2
        for i in 0..h {
            pixels.extend([px!(f 1), px!(e w - 2)]);

            if i <= h / 2 {
                pixels.push(px!(f 1));
            } else {
                pixels.push(px!(e 1));
            }

            if i < h - 1 {
                pixels.push(px!(nl));
            }
        }

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = AH,
    min_size = (3, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];

        // The symbol of a set of 2 vertical lines, the rightmost which starts just above the middle vertical point
        // Spacing is given by w-2
        for i in 0..h {
            pixels.extend([px!(f 1), px!(e w - 2)]);

            if i > h / 2 {
                pixels.push(px!(f 1));
            } else {
                pixels.push(px!(e 1));
            }

            if i < h - 1 {
                pixels.push(px!(nl));
            }
        }

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = UH,
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        // Top of the mark, and both side lines
        let mut pixels = vec![px!(f 1), px!(e w - 4), px!(f 1), px!(e 1), px!(f 1)];

        // Top half adds the right mark
        for _ in 0..((h - 2) - (h / 2)) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 1), px!(f 1),
            ]);
        }

        // Bottom half completes the left shape, accent mark blank
        for _ in 0..(h / 2) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 2),
            ]);
        }

        // Finally a line of w-4 pixels, with a 1px margin
        pixels.extend([
            px!(nl),
            px!(e 1), px!(f w - 4), px!(e 3),
        ]);

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = U,
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        // Both side lines
        let mut pixels = vec![px!(f 1), px!(e w - 4), px!(f 1), px!(e 2)];

        // Top half completes the left shape, accent mark blank
        for _ in 0..(h / 2) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 2),
            ]);
        }

        // Bottom half adds the right mark
        for _ in 0..((h - 2) - (h / 2)) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 1), px!(f 1)
            ]);
        }

        // Finally a line of w-4 pixels, with a 1px margin, and mark
        pixels.extend([
            px!(nl),
            px!(e 1), px!(f w - 4), px!(e 2), px!(f 1)
        ]);
        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = I,
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        // A line of w-4 pixels, with a 1px margin, and the mark
        let mut pixels = vec![px!(e 1), px!(f w - 4), px!(e 2), px!(f 1)];

        // Top half adds the right mark
        for _ in 0..((h - 2) - (h / 2)) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 1), px!(f 1)
            ]);
        }

        // Bottom half completes the left shape, accent mark blank
        for _ in 0..=(h / 2) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 2)
            ]);
        }

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = E,
    min_size = (5, 3),
    height_fungible = true,
    |w, h| {
        // A line of w-4 pixels, with a 1px margin
        let mut pixels = vec![px!(e 1), px!(f w - 4), px!(e 3)];

        // Top half completes the left shape, accent mark blank
        for _ in 0..(h / 2) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 2)
            ]);
        }

        // Bottom half adds the right mark
        for _ in 0..=((h - 2) - (h / 2)) {
            pixels.extend([
                px!(nl),
                px!(f 1), px!(e w - 4), px!(f 1), px!(e 1), px!(f 1)
            ]);
        }

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = O,
    min_size = (3, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![
            px!(e 1), px!(f w - 2), px!(e 1), px!(nl), 
        ];

        // A pair of lines, far left and far right
        for _ in 0..(h - 2) {
            pixels.extend([px!(f 1), px!(e w - 2), px!(f 1), px!(nl)]);
        }

        // A line of w-2 pixels, with a 1px margin
        pixels.extend([px!(e 1), px!(f w - 2), px!(e 1)]);

        pixels
    }
);

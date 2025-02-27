#![allow(unused_variables)]
use crate::glyphs::special::*;

#[rustfmt::skip]
glyph!(
    glyph = Unknown,
    min_size = (5, 9),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![
            px!(f w), px!(nl), //                                             █████
            px!(f 1), px!(e 3), px!(f w-4), px!(nl), //                       █   █
            px!(f 1), px!(e 1), px!(f 1), px!(e 1), px!(f w-4), px!(nl), //   █ █ █
            px!(f 3), px!(e 1), px!(f w-4), px!(nl), //                       ███ █
            px!(f 2), px!(e 1), px!(f w-3), px!(nl), //                       ██ ██
            px!(f 2), px!(e 1), px!(f w-3), px!(nl), //                       ██ ██
        ];

        for _ in 0..(h - 9) {
            pixels.extend([px!(f 2), px!(e 1), px!(f w-3), px!(nl), ]);//     ██ ██
        }

        pixels.extend([px!(f w), px!(nl), //                                  █████
            px!(f 2), px!(e 1), px!(f w-3), px!(nl), //                       ██ ██
            px!(f w), //                                                      █████
        ]);

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = Spacer,
    min_size = (7, 3),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![];

        let center_w = if w % 2 == 0 { w-5 } else { w-6 };

        for i in 0..h {
            if i != 0 {
                pixels.push(px!(nl));
            }

            if i % 2 == 0 {
                pixels.extend([px!(f 1), px!(e 1), px!(f 1), px!(e center_w), px!(f 1), px!(e 1), px!(f 1)]);
            } else {
                pixels.extend([px!(e 2), px!(f 1), px!(e center_w), px!(f 1), px!(e 2)]);
            }
        }

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = WordStop,
    min_size = (1, 5),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![
            px!(e 1), px!(nl), 
            px!(f 1), px!(nl)];

        for _ in 0..(h - 4) {
            pixels.extend([px!(e 1), px!(nl)]);
        }

        pixels.extend([px!(f 1), px!(nl), px!(e 1)]);
        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = SentenceStop,
    min_size = (3, 5),
    height_fungible = true,
    |w, h| { 
        let mut pixels = vec![
            px!(e 3), px!(nl), 
            px!(f 1), px!(e 1), px!(f 1), px!(nl)];

        for _ in 0..(h - 4) {
            pixels.extend([px!(e 3), px!(nl)]);
        }

        pixels.extend([px!(f 1), px!(e 1), px!(f 1), px!(nl), px!(e 3)]);
        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = Deific,
    min_size = (4, 9),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![
            //
            // On top we have the dots indicating a modifier sound
            px!(e 1), px!(f 1), px!(e 1), px!(f 1), px!(nl),
            px!(nl),
            //
            // Next a line
            px!(e 1), px!(f w - 1), px!(nl),
            //
            // Continuing the vertical line to the first crossbar
            px!(e 1), px!(f 1), px!(nl),
            px!(f 3), px!(nl),
            //
            // The 2nd crossbar
            px!(e 1), px!(f 1), px!(nl),
            px!(e 1), px!(f 2),
        ];

        // Now vertical line until we reach h
        for _ in 0..(h - 7) {
            pixels.extend([px!(nl), px!(e 1), px!(f 1)]);
        }

        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = Posessive,
    min_size = (6, 10),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![
            //
            // On top we have the vertical bar, then the first dot
            px!(e 3), px!(f 1), px!(nl),
            px!(e 1), px!(f 1), px!(e 1), px!(f 1), px!(nl),
        ];

        // At min size, there is one line segment here - add another for each additional height
        for _ in 0..(h - 9) {
            pixels.extend([px!(e 3), px!(f 1), px!(nl)]);
        }

        // The next dot
        pixels.extend([px!(e 1), px!(f 1), px!(e 1), px!(f 1), px!(nl)]);

        // The next line segment
        pixels.extend([px!(e 3), px!(f 1), px!(nl)]);

        // The first crossbar
        pixels.extend([px!(e 2), px!(f 2), px!(nl)]);

        // The next line segment
        pixels.extend([px!(e 3), px!(f 1), px!(nl)]);

        // The last crossbar
        pixels.extend([px!(e 1), px!(f 5), px!(nl)]);

        // The next line segment
        pixels.extend([px!(e 3), px!(f 1), px!(nl)]);

        // And the closer
        pixels.push(px!(f 4));
        pixels
    }
);

#[rustfmt::skip]
glyph!(
    glyph = Honourific,
    min_size = (5, 8),
    height_fungible = true,
    |w, h| {
        let mut pixels = vec![px!(f 1), px!(e 1), px!(f 2), px!(nl)];

        // The top and bottom have fungible segments, lets divide the extra height between them
        let extra_top = (h - 8) / 2;
        let extra_bottom = h - 8 - extra_top;

        // Repeat the last line
        for _ in 0..=extra_top {
            pixels.extend([px!(e 2), px!(f 1), px!(e 1), px!(f 1), px!(nl)]);
        }

        // Contains the end of the TR line and the start of the ML line
        pixels.extend([
            px!(f 1), px!(e 1), px!(f 1), px!(e 1), px!(f 1), px!(nl),
        ]);

        // Center
        pixels.extend([
            px!(e 1), px!(f 2), px!(nl),
            px!(e 2), px!(f 2), px!(nl),
        ]);

        // Contains the end of the BL line and the MR line
        pixels.extend([
            px!(f 1), px!(e 1), px!(f 1), px!(e 1), px!(f 1), px!(nl),
        ]);

        // The bottom fungible
        for _ in 0..=extra_bottom {
            pixels.extend([px!(f 1), px!(e 1), px!(f 1), px!(nl)]);
        }

        // And the bottom horn
        pixels.extend([px!(e 1), px!(f 2), px!(e 1), px!(f 1), px!(nl)]);

        pixels
    }
);

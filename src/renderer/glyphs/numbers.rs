use super::*;
/// Generates a phonoglyph for a given number
///
/// ```text
/// ███████████
/// █         █
/// █ █ █     █
/// █ █       █
/// █         █
/// █ █ █ █ █ █
/// █ █ █     █
/// █         █
/// █ █       █
/// █         █
/// ███████████
/// ```
pub fn numeric_phonoglyph(mut value: u16) -> GlyphRenderer {
    // The number will contain a set of rows, each representing ascending powers of 10
    // The rows have a set of dots (value 1), then a set of lines (value 3) which are added together

    let mut num_rep: Vec<(u16, u16)> = vec![];
    let mut divisor = 10u32;
    let mut longest_row = 0;
    while value > 0 {
        let removed = u32::from(value) % divisor;
        let digit = removed / (divisor / 10);
        value -= removed as u16;
        divisor *= 10;

        if digit == 0 {
            num_rep.push((0, 0));
            continue;
        }

        let ones = (digit % 3) as u16;
        let threes = ((digit - u32::from(ones)) / 3) as u16;
        num_rep.push((ones, threes));

        let length = ones + threes;
        if length > longest_row {
            longest_row = length;
        }
    }

    // Min width is given by (3 + 2w) where w is width of the longest row
    let min_width = 3 + 2 * longest_row;

    // Height is more complex and depends on the presence of 3s in a row
    let mut min_height = 4;
    for (_, threes) in &num_rep {
        min_height += if *threes > 0 { 3 } else { 2 };
    }

    let render = move |glyph: &GlyphRenderer, w: u16, h: u16| {
        let num_rep = num_rep.clone();
        let mut pixels = vec![];
        let (min_width, _) = glyph.min_size;

        let lpadding = w - min_width;
        let rpadding = w - min_width - lpadding;

        // First row is a line of w pixels
        pixels.push(n_pixels(w, true));
        pixels.push(LINEBREAK);

        // Empty row
        pixels.push(n_pixels(1, true));
        pixels.push(n_pixels(w - 2, false));
        pixels.push(n_pixels(1, true));
        pixels.push(LINEBREAK);

        // Divide the logical rows into physical rows
        let mut rows = vec![];
        for (ones, threes) in &num_rep {
            rows.push(ones + threes);
            if *threes > 0 {
                rows.push(*threes);
            }

            rows.push(255);
        }

        // Render the rows
        let n_rows = rows.len() as u16;
        for row in rows {
            if row == 255 {
                pixels.push(n_pixels(1, true));
                pixels.push(n_pixels(w - 2, false));
                pixels.push(n_pixels(1, true));
                pixels.push(LINEBREAK);
                continue;
            } else if row == 0 {
                pixels.push(n_pixels(1, true));
                pixels.push(n_pixels(lpadding + 1, false));
                pixels.push(n_pixels(w - lpadding - rpadding - 4, true));
                pixels.push(n_pixels(rpadding + 1, false));
                pixels.push(n_pixels(1, true));
                pixels.push(LINEBREAK);
                continue;
            }

            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(lpadding + 1, false));
            for _ in 0..row {
                pixels.push(n_pixels(1, true));
                pixels.push(n_pixels(1, false));
            }

            let r2padding = w - (row * 2) + rpadding - 3;
            if r2padding > 0 {
                pixels.push(n_pixels(r2padding, false));
            }
            pixels.push(n_pixels(1, true));
            pixels.push(LINEBREAK);
        }

        // If rows + 4 < h, add empty rows
        let consumed = n_rows + 4;
        for _ in 0..(h - consumed) {
            pixels.push(n_pixels(1, true));
            pixels.push(n_pixels(w - 2, false));
            pixels.push(n_pixels(1, true));
            pixels.push(LINEBREAK);
        }

        // Last row is a line of w pixels
        pixels.push(n_pixels(w, true));
        pixels.push(LINEBREAK);

        pixels
    };

    GlyphRenderer {
        pronounciation: "",
        ipa_symbol: "",
        min_size: (min_width, min_height),
        height_fungible: true,
        render: RenderingFunction::Dynamic(Box::new(render)),
    }
}

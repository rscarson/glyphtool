use crate::{
    glyphs::numeric::*,
    renderer::{shrtstop::ToShrtstop, Renderer},
};

impl Renderer for Number {
    fn min_size(&self) -> (u32, u32) {
        let mut longest_row = 0;
        let mut rows_with_3 = 0;
        for (ones, threes) in self.rows() {
            let length = ones + threes;
            if length > longest_row {
                longest_row = length;
            }

            if *threes > 0 {
                rows_with_3 += 1;
            }
        }

        // Min width is given by (3 + 2w) where w is width of the longest row
        let min_width = 3 + 2 * longest_row;

        // Height is more complex and depends on the presence of 3s in a row
        let rows_without_3 = self.rows().len() - rows_with_3;
        let min_height = 4 + (rows_with_3 * 3) + (rows_without_3 * 2);

        (min_width, min_height as u32)
    }

    fn height_fungible(&self) -> bool {
        true
    }
}
impl ToShrtstop for Number {
    fn to_shrtstop(&self, w: u32, h: u32) -> Vec<u32> {
        let (w, h) = self.size(w, h);

        let mut pixels = vec![];
        let (min_width, _) = self.min_size();

        let lpadding = w - min_width;
        let rpadding = w - min_width - lpadding;

        // First row is a line of w pixels
        pixels.push(px!(f w));
        pixels.push(px!(nl));

        // Empty row
        pixels.push(px!(f 1));
        pixels.push(px!(e w - 2));
        pixels.push(px!(f 1));
        pixels.push(px!(nl));

        // Divide the logical rows into physical rows
        let mut rows = vec![];
        for (ones, threes) in self.rows() {
            rows.push(ones + threes);
            if *threes > 0 {
                rows.push(*threes);
            }

            rows.push(255);
        }

        // Render the rows
        let n_rows = rows.len() as u32;
        for row in rows {
            if row == 255 {
                pixels.push(px!(f 1));
                pixels.push(px!(e w - 2));
                pixels.push(px!(f 1));
                pixels.push(px!(nl));
                continue;
            } else if row == 0 {
                pixels.push(px!(f 1));
                pixels.push(px!(e lpadding + 1));
                pixels.push(px!(f w - lpadding - rpadding - 4));
                pixels.push(px!(e rpadding + 1));
                pixels.push(px!(f 1));
                pixels.push(px!(nl));
                continue;
            }

            pixels.push(px!(f 1));
            pixels.push(px!(e lpadding + 1));
            for _ in 0..row {
                pixels.push(px!(f 1));
                pixels.push(px!(e 1));
            }

            let rpadding_2 = w - (row * 2) + rpadding - 3;
            if rpadding_2 > 0 {
                pixels.push(px!(e rpadding_2));
            }
            pixels.push(px!(f 1));
            pixels.push(px!(nl));
        }

        // If rows + 4 < h, add empty rows
        let consumed = n_rows + 4;
        for _ in 0..=(h - consumed) {
            pixels.push(px!(f 1));
            pixels.push(px!(e w - 2));
            pixels.push(px!(f 1));
            pixels.push(px!(nl));
        }

        // Last row is a line of w pixels
        pixels.push(px!(f w));

        pixels
    }
}

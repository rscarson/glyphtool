//! Special definition for the numeric glyphs
use crate::renderer::render_trait::{RenderRow, Renderer};

use super::Glyph;

/// Represents a given number
/// - Dots are 1, lines are 3, A whole row is 0
/// - Each row down is a power of 10 higher than the last
///
/// ```text
/// █████████████
/// █           █
/// █  █ █      █
/// █  █        █
/// █           █
/// █  █ █ █ █  █
/// █  █ █      █
/// █           █
/// █  █        █
/// █           █
/// █████████████
/// ```
#[derive(Clone)]
pub struct Number {
    render: Vec<RenderRow>,
    stretch_rows: [usize; 2],
    value: u32,
}
impl Glyph for Number {
    fn pronounciation(&self) -> &'static str {
        ""
    }

    fn as_boxed(&self) -> Box<dyn Glyph> {
        Box::new(self.clone())
    }
}
impl Renderer for Number {
    fn render_inner(&self) -> &[RenderRow] {
        &self.render
    }

    fn stretch_columns(&self) -> &[usize] {
        &[]
    }

    fn stretch_rows(&self) -> &[usize] {
        &self.stretch_rows
    }
}

impl Number {
    /// Create a new number glyph
    #[must_use]
    pub fn new(value: u32) -> Self {
        let mut digits = vec![];

        //
        // Starting at 10, mod at powers of ten to extract digits
        let mut divisor = 10;
        let mut remainder = value;
        while remainder > 0 {
            let removed = remainder % divisor;
            let digit = removed / (divisor / 10);

            digits.push(digit);
            remainder -= removed;
            divisor *= 10;
        }

        //
        // Each digit corresponds to a pre-rendered section below
        // We mark 0 digit rows for later processing
        let mut render: Vec<RenderRow> = vec![];
        let mut nil_rows = vec![];
        for (i, digit) in digits.iter().enumerate() {
            match digit {
                1 => render.extend_from_slice(ROW_1),
                2 => render.extend_from_slice(ROW_2),
                3 => render.extend_from_slice(ROW_3),
                4 => render.extend_from_slice(ROW_4),
                5 => render.extend_from_slice(ROW_5),
                6 => render.extend_from_slice(ROW_6),
                7 => render.extend_from_slice(ROW_7),
                8 => render.extend_from_slice(ROW_8),
                9 => render.extend_from_slice(ROW_9),

                _ => {
                    nil_rows.push((4 * i) + 2); // 4 rows per digit - 2 and 3 will be modified

                    render.extend_from_slice(ROW_0);
                }
            }
        }

        //
        // Now we get the intended width, pad the rows,
        // And draw lines for all nil rows
        let width = render.iter().map(RenderRow::len).max().unwrap_or_default();
        for (i, row) in render.iter_mut().enumerate() {
            let mut inner_row = row.to_owned();

            let mut padding = width - row.len();
            if nil_rows.contains(&i) {
                // Pad to width with 1s
                inner_row.extend(std::iter::repeat(1).take(padding));
                padding = 0;
            }

            // Pad with 0s to width + 2
            inner_row.extend(std::iter::repeat(0).take(padding + 2));

            // Add right wall
            inner_row.push(1);

            *row = RenderRow::Dynamic(inner_row);
        }

        //
        // Add header
        let mut header = vec![1];
        header.extend(std::iter::repeat(1).take(width + 1));
        header.push(1);
        render.insert(0, RenderRow::Dynamic(header.clone()));

        // 2 empty lines...
        let mut blank = vec![1];
        blank.extend(std::iter::repeat(0).take(width + 1));
        blank.push(1);
        render.push(RenderRow::Dynamic(blank.clone()));
        render.push(RenderRow::Dynamic(blank));

        //
        // Add footer
        render.push(RenderRow::Dynamic(header));

        //
        // Calculate stretch points
        let stretch_rows = [render.len() - 2, 1];

        Self {
            render,
            stretch_rows,
            value,
        }
    }

    /// Get the value of the number
    #[must_use]
    pub fn value(&self) -> u32 {
        self.value
    }
}

#[rustfmt::skip]
const ROW_0: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0]),
    RenderRow::Static(&[1, 0, 0])
];
#[rustfmt::skip]
const ROW_1: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 0])
];
#[rustfmt::skip]
const ROW_2: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 0, 0, 0, 0])
];
#[rustfmt::skip]
const ROW_3: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 0])
];
#[rustfmt::skip]
const ROW_4: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 0])
];
#[rustfmt::skip]
const ROW_5: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 0, 0, 0, 0])
];
#[rustfmt::skip]
const ROW_6: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1])
];
#[rustfmt::skip]
const ROW_7: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 0])
];
#[rustfmt::skip]
const ROW_8: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0])
];
#[rustfmt::skip]
const ROW_9: &[RenderRow] = &[
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 0]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 1]),
    RenderRow::Static(&[1, 0, 0, 1, 0, 0, 1, 0, 0, 1])
];

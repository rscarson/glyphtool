//! Module dealing with the physical appearance of the glyphs in the alphabet.
//! Contains renderers for each glyph, and for various groups of glyphs.

#[macro_use]
pub mod shrtstop;

pub mod bitmap;

//pub mod glyphs;
pub mod utilities;

pub mod render_trait;

mod block;
mod row;
mod stack;

pub use block::{GlyphBlockOptions, GlyphBlockRenderer};
pub use row::GlyphRowRenderer;
pub use stack::GlyphStackRenderer;

macro_rules! impl_renderer {
    (
        $for:path,
        glyph = [ $([ $($px:literal),+ ]),+ $(,)?],
        vstretch = [$($v:literal),*],
        hstretch = [$($h:literal),*]
    ) => {
        impl $crate::renderer::render_trait::Renderer for $for {
            fn render_inner(&self) -> &[$crate::renderer::render_trait::RenderRow] {
                &[
                    $(
                        $crate::renderer::render_trait::RenderRow::Static(&[$($px),+]),
                    )+
                ]
            }

            fn stretch_columns(&self) -> &[usize] {
                &[ $($h,)* ]
            }

            fn stretch_rows(&self) -> &[usize] {
                &[ $($v,)* ]
            }
        }
    };
}

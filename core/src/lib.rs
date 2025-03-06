//! Core functionality for glyphtool
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_wrap)]

#[macro_use]
pub mod renderer;

pub mod database;
pub mod error;
pub mod glyphs;
pub mod lexer;
pub mod postprocessor;

mod database;
mod error;
mod lexer;
mod postprocessor;
mod renderer;

mod autophononimbus;
mod commands;

fn main() {
    if let Err(e) = commands::run() {
        eprintln!("{e}");
    }
}

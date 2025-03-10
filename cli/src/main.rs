mod commands;
mod dictionary;

fn main() {
    if let Err(e) = commands::run() {
        eprintln!("{e}");
    }
}

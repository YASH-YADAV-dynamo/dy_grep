use std::{env, process};
use rust_grep::Config;

fn main() {
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    if let Err(err) = rust_grep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
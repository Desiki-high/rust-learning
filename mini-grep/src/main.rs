use std::env;
use std::process;

pub mod config;

// command like the ./mini_grep content filepath
fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = config::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

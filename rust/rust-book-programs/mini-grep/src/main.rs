use std::env;
use std::process::exit;

use minigrep::{run, Config};

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("An error occurred: {error}");
        exit(1);
    }
}

use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        minigrep::print_error("Problem parsing arguments", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        minigrep::print_error("Application error", e);
    };
}

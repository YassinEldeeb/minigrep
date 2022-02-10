use minigrep::Config;
use std::{env, process};

fn main() {
    // Setup `term` for supporting non-ANSI terminals
    term::stdout();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    let config = Config::new(env::args(), case_sensitive).unwrap_or_else(|err| {
        minigrep::print_error("Problem parsing arguments", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        minigrep::print_error("Application error", e);
    };
}

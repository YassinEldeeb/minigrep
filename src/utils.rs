use colored::Colorize;
use std::fmt::Display;

use crate::Config;

pub fn print_error<T, U>(error_type: T, error_msg: U)
where
    T: Display + Colorize,
    U: Display,
{
    let error = format!("{}{} {}", error_type.red(), ":".bold(), error_msg);
    eprintln!("{}", error);
}

pub fn print_success_msg(config: Config, num_of_matches: usize) {
    println!(
        "{} {} {} `{}` {} {}{}",
        "Found".bright_white(),
        num_of_matches.to_string().bright_cyan().bold(),
        "Matches for".bright_white(),
        config.query.bold().bright_purple(),
        "in".bright_white(),
        config.file_location.bold().bright_cyan(),
        ":".bright_white()
    );
}

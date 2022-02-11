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

/// Returns a `String` for the formatted help message
/// when a user doesn't use the CLI properly with the correct arguments.
/// # Example
///
/// ```rust
/// use minigrep;
///
/// println!("{}", minigrep::help_msg());
/// ```
///
/// # Result in terminal
///
/// ```sh
/// $ cargo run
/// Problem parsing arguments: not enough arguments
/// Usage: minigrep [QUERY] [FILE_PATH] `i`        
/// Info: [Required] `Optional`
/// ```
pub fn help_msg() -> String {
    let msg = format!(
        "not enough arguments\n{} {} {} {} {}\n{} {} {}",
        "Usage:".cyan(),
        "minigrep".bright_purple(),
        "[QUERY]".green(),
        "[FILE_PATH]".green(),
        "`i`".yellow(),
        "Info:".cyan(),
        "[Required]".green(),
        "`Optional`".yellow(),
    );
    msg
}

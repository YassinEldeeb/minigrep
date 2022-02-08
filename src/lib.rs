use colored::Colorize;
use std::{error::Error, fmt::Display, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_location: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let file_location = &args[2];

        Ok(Config {
            query,
            file_location,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.file_location)?;

    println!("{}", file);

    Ok(())
}

pub fn print_error<T, U>(error_type: T, error_msg: U)
where
    T: Display + Colorize,
    U: Display,
{
    let error = format!("{}{} {}", error_type.red(), ":".bold(), error_msg);
    println!("{}", error);
}

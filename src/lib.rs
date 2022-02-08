use std::{error::Error, fs};
mod utils;
pub use utils::*;

#[derive(Eq, PartialEq, Debug)]
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

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];

    for line in content.lines() {
        if line.contains(query) {
            matches.push(line)
        }
    }

    matches
}

#[cfg(test)]
mod search_functionality {
    use super::*;

    #[test]
    fn it_searches() {
        let query = "Rust";
        let content = "\
The most loved programming languages are:
1. Typescript.
2. the Rust programming langauge.
3. Kotlin.
        ";

        assert_eq!(
            search(query, content),
            ["2. the Rust programming langauge."]
        );
    }
}

#[cfg(test)]
mod config_struct {
    use crate::Config;

    #[test]
    fn not_enough_args() {
        let args = [0.to_string(), 1.to_string()];
        if let Err(msg) = Config::new(&args) {
            assert_eq!(msg, "not enough arguments")
        }
    }

    #[test]
    fn instantiate_new_config() {
        let query = "word";
        let file_location = "hello.txt";
        let args = [
            "0".to_string(),
            query.to_string(),
            file_location.to_string(),
        ];

        if let Ok(config) = Config::new(&args) {
            let equivalent = Config {
                query,
                file_location,
            };
            assert_eq!(config, equivalent)
        }
    }
}

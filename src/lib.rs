use std::{env::Args, error::Error, fs};
mod core;
mod utils;
pub use utils::*;

#[derive(Eq, PartialEq, Debug)]
pub struct Config {
    query: String,
    file_location: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args, env_case_sensitive: bool) -> Result<Config, String> {
        // Skip first arg which is binary name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(help_msg()),
        };
        let file_location = match args.next() {
            Some(arg) => arg,
            None => return Err(help_msg()),
        };

        let case_sensitive = args.next();

        let case_sensitive = if let Some(v) = case_sensitive {
            v != "i"
        } else {
            env_case_sensitive
        };

        Ok(Config {
            query,
            file_location,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&config.file_location)?;

    let matches = if config.case_sensitive {
        core::search(&config.query, &file)
    } else {
        core::search_insensitive(&config.query, &file)
    };

    let colorized_matches = core::colorize_matches(matches);

    utils::print_success_msg(config, colorized_matches.len());

    for line in colorized_matches {
        println!("{}", line)
    }

    Ok(())
}

#[cfg(test)]
mod config_struct {
    use std::env::Args;

    use crate::Config;

    #[test]
    fn not_enough_args() {
        let args = [0.to_string(), 1.to_string()];
        if let Err(msg) = Config::new(&args, false) {
            assert!(msg.contains("not enough arguments"))
        }
    }

    #[test]
    fn arguments_override_envs() {
        let query = "word";
        let file_location = "hello.txt";
        let args = [
            0.to_string(),
            query.to_string(),
            file_location.to_string(),
            "sensitive".to_string(),
        ];

        // If env=true & arg=false
        // arg should take the precedence
        if let Ok(config) = Config::new(&args, false) {
            let equivalent = Config {
                query,
                file_location,
                case_sensitive: true,
            };
            assert_eq!(config, equivalent);
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

        if let Ok(config) = Config::new(&args, false) {
            let equivalent = Config {
                query,
                file_location,
                case_sensitive: false,
            };
            assert_eq!(config, equivalent);
        }
        if let Ok(config) = Config::new(&args, true) {
            let equivalent = Config {
                query,
                file_location,
                case_sensitive: true,
            };
            assert_eq!(config, equivalent);
        }
    }
}

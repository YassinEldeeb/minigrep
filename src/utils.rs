use colored::Colorize;
use std::fmt::Display;

pub fn print_error<T, U>(error_type: T, error_msg: U) -> String
where
    T: Display + Colorize,
    U: Display,
{
    let error = format!("{}{} {}", error_type.red(), ":".bold(), error_msg);
    println!("{}", error);

    error
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn format_error_properly() {
        let err_msg = print_error("Memory Leak", "something went wrong");

        assert_debug_snapshot!(err_msg);
    }
}

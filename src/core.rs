use colored::Colorize;

pub fn search(query: &str, content: &str) -> Vec<String> {
    let mut matches: Vec<String> = vec![];

    for (idx, line) in content.lines().enumerate() {
        if line.contains(query) {
            let colored = str::replace(line, query, &query.bright_purple().bold().to_string());

            let formatted = format!("{}. {}", (idx + 1).to_string(), colored);
            matches.push(formatted);
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn gets_matching_lines() {
        let query = "Rust";
        let content = "\
The most loved programming languages are:
1. Typescript.
2. the Rust programming langauge.
3. Kotlin.
        ";

        assert_debug_snapshot!(search(query, content));
    }

    #[test]
    fn gets_nothing_when_no_matches() {
        let query = "Rust";
        let content = "How is it going?";

        assert_eq!(search(query, content), vec![] as Vec<String>);
    }
}

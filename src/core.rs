use colored::Colorize;

#[derive(PartialEq, Eq, Debug)]
pub struct Match<'a> {
    index: usize,
    line: &'a str,
    matched_query: &'a str,
}

impl<'a> Match<'a> {
    fn new(matched_query: &'a str, line: &'a str, index: usize) -> Self {
        Match {
            matched_query,
            line,
            index,
        }
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<Match<'a>> {
    let mut matches: Vec<Match> = vec![];

    for (idx, line) in content.lines().enumerate() {
        if line.contains(query) {
            matches.push(Match::new(query, line, idx));
        }
    }

    matches
}

pub fn search_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<Match<'a>> {
    let mut matches: Vec<Match> = vec![];

    for (idx, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            let start_idx_of_match = line.to_lowercase().find(&query.to_lowercase()).unwrap();
            let matched_query = &line[start_idx_of_match..(start_idx_of_match + query.len())];

            matches.push(Match::new(matched_query, line, idx));
        }
    }

    matches
}

pub fn colorize_matches(matches: Vec<Match>) -> Vec<String> {
    let mut colorized: Vec<String> = Vec::new();

    for e in matches {
        let colored = str::replace(
            e.line,
            e.matched_query,
            &e.matched_query.bright_purple().bold().to_string(),
        );

        let formatted = format!("{}. {}", (e.index + 1).to_string(), colored);
        colorized.push(formatted);
    }

    colorized
}

#[cfg(test)]
mod tests {
    use super::*;

    const QUERY: &str = "programming";
    const CONTENT: &str = "\
The most loved programming languages are:
1. Typescript.
2. The Rust Programming Langauge.
3. Kotlin.
        ";

    #[test]
    fn search_case_insensitive() {
        assert_eq!(
            search_insensitive(QUERY, CONTENT),
            vec![
                Match::new(
                    "programming",
                    "The most loved programming languages are:",
                    0
                ),
                Match::new("Programming", "2. The Rust Programming Langauge.", 2)
            ]
        );
    }

    #[test]
    fn search_case_sensitive() {
        assert_eq!(
            search(QUERY, CONTENT),
            vec![Match::new(
                "programming",
                "The most loved programming languages are:",
                0
            )]
        );
    }

    #[test]
    fn gets_nothing_when_no_matches() {
        let query = "Rust";
        let content = "How is it going?";

        assert_eq!(search(query, content), vec![] as Vec<Match>);
    }
}

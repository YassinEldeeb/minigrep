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
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(index, line)| Match::new(query, line, index))
        .collect()
}

pub fn search_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<Match<'a>> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|(index, line)| {
            let start_idx_of_match = line.to_lowercase().find(&query.to_lowercase()).unwrap();
            let matched_query = &line[start_idx_of_match..(start_idx_of_match + query.len())];

            Match::new(matched_query, line, index)
        })
        .collect()
}

pub fn colorize_matches(matches: Vec<Match>) -> Vec<String> {
    matches
        .into_iter()
        .map(|e| {
            let colored = str::replace(
                e.line,
                e.matched_query,
                &e.matched_query.bright_purple().bold().to_string(),
            );

            format!("{}. {}", (e.index + 1).to_string(), colored)
        })
        .collect()
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

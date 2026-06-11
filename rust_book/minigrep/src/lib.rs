pub fn search<'a>(
    query: &str,
    contents: &'a str,
    ignore_case: bool,
) -> impl Iterator<Item = &'a str> {
    let query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    contents.lines().filter(move |line| {
        if ignore_case {
            line.to_lowercase().contains(&query)
        } else {
            line.contains(&query)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\"
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false).collect::<Vec<_>>()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(query, contents, true).collect::<Vec<_>>()
        );
    }
}

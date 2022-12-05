use std::error::Error;
use std::fs;

use crate::config::Config;

pub fn search_word_in_file(config: Config) -> Result<(), Box<dyn Error>> {
    let path = config.file_path();
    let pat = &config.query();
    let content = &fs::read_to_string(path)
        .expect("Error opening file");

    let result = if config.case_sensitive() {
        search_case_sensitive(pat, content)
    } else {
        search_case_insensitive(pat, content)
    };

    if result.len() > 0 {
        for line in result {
            println!("{}", line);
        }
    } else {
        println!("No results found");
    }

    Ok(())
}


fn search_case_sensitive<'a>(pat: &'a str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(&pat)).collect()
}

fn search_case_insensitive<'a>(pat: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.to_lowercase().contains(&pat.to_lowercase())).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let expected = vec!["Rust:", "Trust me."];
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            expected,
            search_case_insensitive(query, contents)
        );
    }
}
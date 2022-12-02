use std::{env, process};

use crate::config::Config;

pub fn get_config(args: Vec<String>) -> Config {
    let config: Config;

    if args.len() > 0 {
        config = Config::new(&args).unwrap_or_else(|err|{
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
        );
    } else{
        config = Config::build(env::args()).unwrap_or_else( |err |{
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
        );
    }
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let input = vec!["program_name".to_string(), "query".to_string(), "file.txt".to_string()];
        let expected = Config::new(&input).unwrap();

        let result = get_config(input);
        assert_eq!(expected, result); // this works b/c of generated partial equality
        assert_eq!(expected.query(), result.query());
        assert_eq!(expected.file_path(), result.file_path());
        assert_eq!(expected.case_sensitive(), result.case_sensitive());

    }
}
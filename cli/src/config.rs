use std::env;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Config {
    query: String,
    file_path: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments. use: -- word file.txt ");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // read env. variable; otherwise set case sensitive to true by default
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }

    /// build is a more comprehensive constructor that checks each argument individually
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {

        args.next(); // skip first argument

        // iterating through each argument allows specific error messages
        let query = match args.next(){
            None => return Err("Didn't get a query string"),
            Some(arg) => arg,
        };

        let file_path = match args.next(){
            None => return Err("Didn't get a file path"),
            Some(arg) => arg,
        };

        let ignore_case = env::var("CASE_INSENSITIVE");
        let case_sensitive = match ignore_case {
            Ok(_) => true,
            Err(_) => false,
        };

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })

    }
}

impl Config{
    pub fn query(&self) -> String{
        self.query.clone()
    }

    pub fn file_path(&self) -> String{
        self.file_path.clone()
}

    pub fn case_sensitive(&self) -> bool {
        self.case_sensitive
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_config() {
        // the first CMD argument is always the program name so we emulate this with "program_name"
        // to prevent a not enough argument panic
        let expected = vec!["program_name".to_string(), "query".to_string(), "file.txt".to_string()];
        let result = Config::new(&expected).unwrap();

        assert_eq!(expected[1], result.query()); // if you cannot compare objects, you compare each field
        assert_eq!(expected[2], result.file_path());
        assert_eq!(true, result.case_sensitive());
    }
}



// _simple_word_search contains a number of problems, among others:
// 1. It does not check if the arguments, query & path actually exist.
// 2. It does not check if the file can actually be opened and read.
// 3. It does not check if the search result contains any values.
use std::fs;

pub fn _simple_word_search(args: Vec<String>){
    // 1) Parse two command line argument, the word to search and the file path
    let word = args[1].to_string();
    let path = args[2].to_string();

    // 2) Read the file from the path into a string
    let content = &fs::read_to_string(&path).unwrap();

    // 3) Search the word in text from the file
    let result: Vec<&str> = content.lines().filter(| line| line.contains(&word)).collect();

    // 4) Print out each line where the word occurs
    for line in result {
        println!("{}", line);
    }
}

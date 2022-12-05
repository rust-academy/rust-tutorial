use std::{env, process};

use crate::search::search_word_in_file;
use crate::utils::get_config;

mod simple;
mod config;
mod utils;
mod search;

// Requirements
// 1) Parse two command line argument, the word to search and the file path
// 2) Read the file from the path into a string
// 3) Search the word in text from the file
// 4) Print out each line where the word occurs
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_config(args);
    if let Err(e) = search_word_in_file(config) {
        eprintln!("Application error: {e}"); // redirects output to stdout i.e. terminal
        process::exit(1);
    }
}

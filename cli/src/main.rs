mod simple;
mod config;
mod utils;

use std::{env, fs};

// Requirements
// 1) Parse two command line argument, the word to search and the file path
// 2) Read the file from the path into a string
// 3) Search the word in text from the file
// 4) Print out each line where the word occurs
fn main() {
     let args: Vec<String> = env::args().collect();
    println!("{:?}", args)
}

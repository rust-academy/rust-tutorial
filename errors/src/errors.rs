use std::{fs, io};
use std::fs::File;
use std::io::Read;

pub fn open_file() {
    // let _f = File::open("Hello.txt").unwrap();

    let _f = File::open("Hello.txt")
        .expect("Failed to open file Hello.txt");
}

pub fn result_match_panic(path: &str) {
    let file = File::open("Hello.txt");
    match file {
        Ok(file) => {}
        Err(e) => panic!("Failed to open {} Error: {}", path, e),
    };
}

/// recoverable_file_open does the following
/// Try to open the file
///      yes => OK
///      no => Figure out error
///              ==> File Not Found Error
///                    create a new file
///                       yes ==> Ok
///                       no  ==> print error & return
///              ==> Any other error
///                 => Panic & abort
pub fn recoverable_file_open(path: &str) {
    let file = File::open("Hello.txt");
    match file {
        Ok(file) => {}
        Err(e) =>
            match e.kind() {
                io::ErrorKind::NotFound => {
                    match File::create(path) {
                        Ok(file) => (), // return the file
                        Err(e) => {
                            println!("Error creating file: {}", e);
                            return;
                        }
                    }
                }
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
    };
}

// Type alias for return type
pub type StringResult = Result<String, io::Error>;

pub fn read_username_from_file(path: &str) -> StringResult {
    let file = File::open(path);

    let mut f = match file {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut s = String::new();
    return match s.read_to_string(&mut f) {
        Ok(_) => Ok(s),
        Err(e) => {
            return Err(e);
        }
    };
}

pub fn read_username_from_file_simplified(path: &str) -> StringResult {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    return Ok(s);
}

pub fn read_file_to_string_simplest(path: &str) -> StringResult {
    fs::read_to_string(path: &str)
}


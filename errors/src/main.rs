mod errors;

const UNRECOVERABLE: bool = false;


fn main() {
    if UNRECOVERABLE {
        unrecoverable();
    } else {
        recoverable();
    }
}

fn unrecoverable() {
    // test_panic();
    // errors::open_file();

    let path = "Hello.txt";
    errors::result_match_panic(path);
}

fn test_panic() {
    panic!("test panic");
}


fn recoverable() {
    let path = "Hello.txt";
    errors::recoverable_file_open(path);
}

//    Ownership rules
// 1. Each value in Rust has a variable that's called its owner.
// 2. here can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.


use std::string;

fn main() {  // main scope
    scope();
    stack_data();
    heap_data();
    function_ownership();
    function_mut_ref();
    change_read_write_ref();
    use_first_word();
} // main scope

fn scope() { // outer scope

    { // inner scope starts  nothing here

        let mut s = "Hello, world!"; // s is valid and in scope

        s.split_whitespace();
    }  // inner scope ends, s will removed from memory
} // outer scope ends

fn stack_data() {
    let x = 5;
    let y = x;

    println!("{}", x);
    println!("{}", y);
}

fn heap_data() {
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;


    println!("{}, world!", s1);
    println!("{}, world!", s2);
    println!("{}, world!", s3);
}

fn function_ownership() {
    let some_string = give_ownership();
    take_ownership(&some_string);
    println!("{}, world!", some_string);
}

fn take_ownership(some_string: &String) {
    println!("{}", some_string);
} // scope ends, some string will be removed

fn give_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // a_string is returned and moves out to the calling function
}

fn function_mut_ref() -> () {
    let main_string = String::from("Hello, ");

    { // inner scope
        let mut mut_string = main_string.clone();
        change_string(&mut mut_string);
    } // scope ends here, mut_string will be removed

    let s = &main_string;

    function_ref(s);
}

fn change_string(mut_string: &mut String) {
    mut_string.push_str("World");
}

fn change_read_write_ref() {
    let mut s = String::from("Something");

    let r1 = &s;
    let r2 = &s;


    println!("{}, {},", r1, r2);

    {
        let r3 = &mut s;
        println!("{},", r3);
    } //
}

fn function_ref(s: &String) {
    // let s1 = String::from("Something");
    //let s = &s1;

    let length = calculate_length_ref(s);
    println!("length: {},", length);

    let cap = calculate_capacity(s);
    println!("capacity: {},", cap);
}


fn calculate_length_ref(s: &String) -> usize { // s is a read-only reference to a String
    return s.len(); // Rust de-ref automatically.
}

fn calculate_capacity(s: &String) -> usize { // s is a read-only reference to a String
    return s.capacity();
}


// You can have either one mutable reference
//
// OR
//
// You can have many immutable references;
//
// But
//
// you cannot have both at the same time.


//  Not strictly a rule, but a best practice widely used in writing Rust functions:
//
// - Take references as argument
// - Return values

fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);
}

fn first_slice() {
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];
}

fn use_first_word() {
    let s = String::from("hello world");

    let first = first_word(&s); //&String ???

    println!("first word: {}", first);
}

fn use_fist_all_string() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string); //&String

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string); // String(!)

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}


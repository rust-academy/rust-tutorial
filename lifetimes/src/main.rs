use std::fmt::Display;

const VERSION: &'static str = "0.1.2.3";

fn main() {
    proper_ref();
    test_longest();
    test_static_lifetime(VERSION);
    test_generic_lifetime(VERSION);
    test_struct_lifetime();
    test_generic_longest();
}

fn _problem(){
    let r: &i32;

    {
        let x = 5;
        r = &x; // ^^ borrowed value does not live long enough
    } // inner scope ends and x goes out of memory

    // println!("{}",r) // dangling pointer
}

fn proper_ref() {
    let x = 5;             // ----------+-- x'b
                                //           |
    let r = &x;           // --+-- r'a | 'a<'b == True
                                //   |       |
    println!("r: {}", r);       //   |       |
                                // --+       |
}                               // ----------+ // x goes out of scope

// when do lifetimes annotations apply?
#[allow(dead_code)]
fn test_1(param_1: Vec<i32>) -> Vec<i32> {
    param_1 // lifetimes don't apply because there are no references in input or output
}

#[allow(dead_code)]
fn test_2(param_1: &Vec<i32>) -> Vec<i32> {
    param_1.clone() // Lifetimes isn't applied because there is no reference in output
}

#[allow(dead_code)]
fn test_3(param_1: &Vec<i32>) -> &Vec<i32> { // '&return_ref<'&param_ref == True
    param_1 // Lifetimes isn't an issue here because there is just one input reference,
} // hence the compiler can infer and annotated lifetimes automatically.

// #[allow(dead_code)]
// fn test_4(param_1: Vec<i32>) -> &'static Vec<i32> {
//     &param_1 //  ^^^^^^^^ returns a reference to data owned by the current function
// } // param_1 is going to be removed from memory



// Lifetimes only apply if there are references
// * in the output of a function
// * and there are more than one reference in the input of a function
// * and if any of the input reference can be returned as output. (!)

// Why?
// Because for a single reference as input, the Rust compiler can infer the lifetimes of the returned reference.
// If there are multiple references as input and any of them can be returned,
// the Rust compiler CANNOT not infer the lifetimes of the output. ==> annotate lifetimes


// Lifetime Annotation Syntax
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else{
        y
    }
}

fn test_longest() {
    let string1 = String::from("hello");
    let string2 = String::from("world!");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest: {}", result);
}

// Static lifetime

fn test_static_lifetime(s: &'static str) {
    println!("{}",s);
}

fn test_generic_lifetime<'a>(s: &'a str) {
    println!("{}",s);
}

// struct lifetime

struct ImportantExcerpt<'a>{
    part: &'a str
}

impl<'a> ImportantExcerpt<'a> {

    fn return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {}", announcement);
        self.part
    }

}

fn test_struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find first sentence.");

    let excerpt = ImportantExcerpt{part: first_sentence};
    let part = excerpt.return_part("What?");

    println!("The first sentence: {}", part);
}

// Lifetime elision rules
// 1. Each parameter that is a reference gets its own lifetime parameter annotation
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
// the lifetime of self is assigned to all output lifetime parameters.

// => Manual lifetime annotation is only required if a function has multiple references as input parameters
// of which any can be returned. Only then the compiler cannot decide which lifetime annotation to assign to the return value

fn longest_with_an_announcement<'a, T, U>(x: &'a T, y: &'a T, ann: &U) ->&'a T
where
    T: PartialOrd,
    U: Display,
    {
        println!("Announcement: {}", ann);
        if x> y{
            x
        } else{
            y
        }
}



fn test_generic_longest(){
    let announcement = "Here comes the sun";
    let x = 32;
    let y = 64;

    let result = longest_with_an_announcement(&x, &y, &announcement);
    println!("{}", result);

    let x = 64.0;
    let y = 1289.0;

    let result = longest_with_an_announcement(&x, &y, &announcement);
    println!("{}", result);

    let x = 'A';
    let y = 'Z';

    let result = longest_with_an_announcement(&x, &y, &announcement);
    println!("{}", result);
}
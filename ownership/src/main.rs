
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
    change_read_write_ref()

} // main scope

fn scope(){ // outer scope

    { // inner scope starts  nothing here

        let mut  s = "Hello, world!"; // s is valid and in scope

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

fn function_ownership(){
    let some_string = give_ownership();
    take_ownership(&some_string);
    println!("{}, world!", some_string);
}

fn take_ownership(some_string: &String){
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
    let mut mut_string = String::from("Hello, ");
    change_string(&mut mut_string);

}


fn change_string(mut_string: &mut String){
    mut_string.push_str("World");
}

fn change_read_write_ref()  {
    let mut s = String::from("Something");

    let r1 = &s;
    let r2 = &s;


   println!("{}, {},", r1, r2);

    {
        let r3 = &mut s;
        println!("{},", r3);
    } //

}

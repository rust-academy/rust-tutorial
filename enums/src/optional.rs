

pub fn test_optional_enum(){

    let some_number = Some(5);
    let no_number = None; // Nil // Null

    println!("Some number: {}", some_number.unwrap());
    println!("No number: {}", no_number.unwrap_or(0));

    // println!("No number: {}", no_number.expect("This number must be present and an integer"));

    let option_number: Option<i32> = None;
    check_number(&option_number);

    let option_number: Option<i32> = Some(41);
    check_number(&option_number);

    let add_one =  plus_one(&option_number);
    println!("Add One number: {}", add_one.unwrap_or(0));

}

fn check_number(option_number: &Option<i32>){
    match option_number {
        Some(number) => { println!("Number: {}", number)},
        None => println!("Nope, there is no number")
    }
}


fn plus_one(option_number: &Option<i32>) -> Option<i32> {
    match option_number {
        None => None,
        Some(i) => Some(i + 1),
    }
}

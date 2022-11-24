fn main() {
    println!("Commons concepts!");
    variables();
    constants();
    shadowing();
    scalars();
    math_operations();
    booleans();
    chars();
    compound();
    conditional_let();
    loop_examples();
}

fn variables() {
    let mut x = 5; // immutable variable
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}

fn constants() {
    // 1) Must annotated type
    // 2) Cannot be mut
    // 3) must be fixed value or a fixed expression

    const SUB_COUNT: u64 = 1000_000;
    const SEC_AS_MIN: u64 = 60 * 60;
}

fn shadowing() {
    let z = 6;
    println!("The value of z is: {z}");

    let z = 42;
    println!("The value of z is: {z}");


    {
        let z = 128;
        println!("The value of z is: {z}");
    }

    let z = 265;
    println!("The value of z is: {z}");
}

fn scalars() {
    let _a = 98222; // Decimal
    let _b = 0xff; // Hex
    let _c = 0o77; // octal
    let _d = 0b1111_0000; // binary
    let _e: u8 = b'A';

    // int overflow
    let _f: u8 = 255; // ok

    // let _g: u8 = 257;
    // overflow wraps back to min. => 1


    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32
}

fn math_operations() {
    let sum = 5 + 10;
    println!("{}", sum);

    let sub = 10 - 7;
    println!("{}", sub);


    // multiplication
    let product = 4 * 30;
    println!("product {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {}", quotient);

    let floored = 2 / 3; // Results in 0
    println!("floored {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder {}", remainder);
}

fn booleans() {
    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation
}

fn chars() {
    // Char
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("heart_eyed_cat {}", heart_eyed_cat);
}

fn compound() {

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
    let sub = tup.1;
    println!("The value of x is: {sub}");

    let _tup2 = (1, 2, 3);

    let (a, b, v) = _tup2;

    // array
    let error_code = [200, 404, 500];

    let not_found = error_code[1];
    println!("The value of not_found is: {not_found}");
}

fn control_flow(number: i32) {
    if number == 0 {
        println!("first condition was true!");
    } else if number < 22 {
        println!("second condition was true!");
    } else if number < 44 {
        println!("third condition was true!");
    } else {
        print!("All previous condition was false ");
    }
}

fn conditional_let() {
    let condition = false;
    let number = if condition { 5 } else { 42 };
    println!("The number is: {}", number);
}

fn loop_examples() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("result = {}", result);

    let mut number = 10;
    while number != 0 {
        println!("number = {}", number);
        number -= 1;
    }
    println!("Liftoff");


    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("{}", element);
    }

    for number in 1..4 {
        println!("{}", number)
    }
}
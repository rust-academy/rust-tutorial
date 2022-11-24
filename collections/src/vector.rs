pub fn test_create_vec() {
    let _a = [1, 2, 3, 4];

    {
        let _v1 = vec![1, 2, 3, 4];
    } // v1 goes out of bounds

    {
        let _v2: Vec<i32> = Vec::new();
    } // v2 goes out of bounds
}

pub fn test_unsafe_vector_access() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // access the third element directly via index
    let third = &v[2];
    println!("Third element: {}", third);

    //let invalid = &v[99];
}

pub fn test_safe_vec_access() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("Handling some type");

    match v.get(2) {
        Some(x) => println!("Found value: {}", x),
        None => println!("Couldn't find value")
    }

    match v.get(99) {
        Some(x) => println!("Found value: {}", x),
        None => println!("Couldn't find value")
    }
}

pub fn test_iterate_vector() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("Immutable iterating...");
    for e in &v {
        println!("{}", e);
    }

    println!("Mutable iterating...");
    for e in &mut v {
        *e += 50;
        println!("{}", e);
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn test_mixed_type_vec() {
    println!("Mixed Type Vector...");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];

    test_all_cell(&row[0]);
    test_all_cell(&row[1]);
    test_all_cell(&row[2]);

    match row.get(2) {
        Some(x) => test_all_cell(x),
        None => println!("Couldn't find value")
    }
}

fn test_all_cell(cell: &SpreadsheetCell) {
    match cell {
        SpreadsheetCell::Int(n) => println!("Integer: {}", n),
        SpreadsheetCell::Float(n) => println!("Float: {}", n),
        SpreadsheetCell::Text(n) => println!("String: {}", n)
    }
}



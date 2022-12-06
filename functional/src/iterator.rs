use std::fmt::Display;

use crate::utils::print_vector;

pub fn test_int_iterator() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7];
    print_vector(&v1);

    // Methods that use the Iterator. For example len.
    // the iterator still exists after calling len until moved out of memory.

    let len = v1.iter().len();
    println!("len: {}", len);

    // Every iterator has a next() methods that returns the next element.
    // Methods that call next are called consuming adaptors,
    // because calling them uses up the iterator to calculate a value based on all elements,
    // for example the sum of all elements in a collection.

    let total = v1.iter().sum::<i32>();
    println!("Total: {}", total);
}

pub fn test_string_iterator() {
    let v1 = vec!["One", "Two", "Three"];
    print_vector(&v1);
    let len = v1.iter().len();
    println!("len: {}", len);
}

pub fn test_iterator_adapters() {
    let v1: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];

    println!("All numbers");
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    print_vector(&v2);

    println!("All EVEN numbers");
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).filter(|x| x % 2 == 0).collect();
    print_vector(&v2);

    println!("All ODD numbers");
    let v2: Vec<i32> = v1.iter().map(|x| x * 3).filter(|x| x % 2 != 0).collect();
    print_vector(&v2);
}

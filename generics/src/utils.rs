use std::fmt::Debug;

// non-generic functions

pub fn print_largest_number(list: &[i32])  {
    let largest = largest_number(list);
    println!("The largest value: {:?}", largest);
}


fn largest_number(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generic functions

pub fn print_largest_value<T: PartialOrd + Copy + Debug>(list: Vec<T>)  {
    let largest = get_largest(list);
    println!("The largest value: {:?}", largest);
}


fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

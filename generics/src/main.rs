mod utils;
mod point;

use point as p;

fn main() {
    largest_value();
    mixed_values();
}

fn largest_value(){
    let number_list = vec![34, 50, 25, 100, 65, 78, 123, 88];
    utils::print_largest_value(number_list);

    let char_list = vec!['y', 'm', 'a', 'q'];
    utils::print_largest_value(char_list);

    let float_list = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];
    utils::print_largest_value(float_list);

    let p1 = p::Point::new(1, 2);
    let p2 = p::Point::new(3, 4);
    let p3 = p::Point::new(5, 7);

    let point_list = vec![p1,p2,p3];
    utils::print_largest_value(point_list);
}

fn mixed_values(){
    let p1 = p::Point::new(1, 2);
    let p2 = p::Point::new(3, 4);
    let p3 = p1.mixup(p2);


    println!("P1: {:?}", p1);
    println!("P2: {:?}", p2);
    println!("P3: {:?}", p3);

}

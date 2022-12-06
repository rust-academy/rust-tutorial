use std::fmt::Display;

use crate::rectangle::{Rectangle, RectangleInventory};
use crate::shirt_inventory::{ShirtColor, ShirtInventory};
use crate::shoe::{Shoe, ShoeInventory, ShoeStyle};

pub fn print_vector<T>(v2: &Vec<T>) where T: Display {
    for val in v2.iter() { println!("Got: {}", val) }
}

pub fn get_shoe_inventory() -> ShoeInventory {
    let shoes = vec![
        Shoe::new(10, ShoeStyle::Sneaker),
        Shoe::new(13, ShoeStyle::Sandal),
        Shoe::new(10, ShoeStyle::Boot),
    ];
    ShoeInventory::new(shoes)
}

pub fn get_rectangles_inventory() -> RectangleInventory {
    let rectangles = vec![
        Rectangle::new(10, 1),
        Rectangle::new(3, 5),
        Rectangle::new(7, 12),
    ];
    RectangleInventory::new(rectangles)
}

pub fn get_shirt_inventory() -> ShirtInventory {
    let shirts = vec![
        ShirtColor::Blue,
        ShirtColor::Red,
        ShirtColor::Blue,
    ];
    return ShirtInventory::new(shirts, 1, 2);
}

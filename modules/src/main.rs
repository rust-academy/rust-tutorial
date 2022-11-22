use std::fmt;
use std::io;

mod garden;

use crate::garden::flowers::{SunFlower, OtherFlower};

fn main() {
    println!("Hello, world!");

    let s1 = SunFlower{};
    let s = OtherFlower{};
}

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

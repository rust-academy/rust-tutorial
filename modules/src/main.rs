use std::fmt;
use std::io;

use crate::garden::flowers::{OtherFlower, SunFlower};

mod garden;

fn main() {
    println!("Hello, world!");

    let s1 = SunFlower {};
    let s = OtherFlower {};
}

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

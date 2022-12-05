use std::fmt::{Debug, Display};

fn _some_function<T: Display + Copy + Clone, U: Clone + Debug>(_t: T, _u: U) -> i32 {
    42
}

fn _some_function_where<T, U>(t: &T, _u: &U) -> T
    where T: Display + Clone,
          U: Clone + Copy + Debug
{
    return t.clone();
}

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

impl<T> Pair<T> where T: Display + PartialOrd {
    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x: {}", self.x);
        } else {
            println!("The largest member is y: {}", self.y);
        }
    }
}
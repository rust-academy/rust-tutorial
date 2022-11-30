#[cfg(test)]
mod test;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

pub fn greetings(name: &str) -> String {
    return format!("Hello, {}!", name);
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        return Rectangle { width, height };
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess value must be greater or equal to 1")
        } else if value > 100 {
            panic!("Guess value must less than or equal to 100")
        }

        return Guess { value };
    }

    fn value(&self) -> i32 {
        self.value
    }
}
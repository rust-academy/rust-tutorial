#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShirtInventory {
    shirts: Vec<ShirtColor>,
    num_red_shirts: i32,
    num_blue_shirts: i32,
}

impl ShirtInventory {
    pub fn new(shirts: Vec<ShirtColor>, num_red_shirts: i32, num_blue_shirts: i32) -> Self {
        ShirtInventory {
            shirts,
            num_red_shirts,
            num_blue_shirts,
        }
    }

    pub fn remaining(&self) -> i32 {
        return self.num_blue_shirts + self.num_red_shirts;
    }

    pub fn most_stocked(&self) -> ShirtColor {
        return if self.num_red_shirts > self.num_blue_shirts {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        };
    }

    pub fn give_away(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        return match user_preference {
            None => {
                let c = self.most_stocked();
                let _ = self.decrement_inventory(c);
                c
            }
            Some(c) => {
                let _ = &self.decrement_inventory(c);
                c
            }
        };
    }

    fn increment_inventory(&mut self, color: ShirtColor) {
        match color {
            ShirtColor::Red => { self.num_red_shirts = self.num_red_shirts + 1 }
            ShirtColor::Blue => { self.num_blue_shirts = self.num_blue_shirts + 1 }
        }
    }

    fn decrement_inventory(&mut self, color: ShirtColor) {
        match color {
            ShirtColor::Red => { self.num_red_shirts = self.num_red_shirts - 1 }
            ShirtColor::Blue => { self.num_blue_shirts = self.num_blue_shirts - 1 }
        }
    }
}

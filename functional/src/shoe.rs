use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum ShoeStyle {
    Sneaker,
    Sandal,
    Boot,
}

impl Display for ShoeStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShoeStyle::Sneaker => write!(f, "Sneaker"),
            ShoeStyle::Sandal => write!(f, "Sandal"),
            ShoeStyle::Boot => write!(f, "Boot")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shoe {
    size: u32,
    style: ShoeStyle,
}

impl Shoe {
    pub fn new(size: u32, style: ShoeStyle) -> Shoe {
        Shoe { size, style }
    }
}

impl Display for Shoe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shoe<Size: {}, Style: {}>", self.size, self.style)
    }
}

pub struct ShoeInventory {
    shoes: Vec<Shoe>,
}

impl ShoeInventory {
    pub fn new(shoes: Vec<Shoe>) -> ShoeInventory {
        ShoeInventory { shoes }
    }

    pub fn shoes_in_size(&self, shoe_size: u32) -> Vec<Shoe> {
        self.shoes.iter().filter(|s| s.size == shoe_size).cloned().collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = utils::get_shoe_inventory();
        let in_my_size = shoes.shoes_in_size(10);

        assert_eq!(in_my_size, vec![
            Shoe {
                size: 10,
                style: ShoeStyle::Sneaker,
            },
            Shoe {
                size: 10,
                style: ShoeStyle::Boot,
            },
        ]);
    }
}
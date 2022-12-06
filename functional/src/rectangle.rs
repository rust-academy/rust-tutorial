use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle<Width: {}, Height: {}>", self.width, self.height)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RectangleInventory {
    rectangles: Vec<Rectangle>,
}

impl RectangleInventory {
    pub fn new(rectangles: Vec<Rectangle>) -> RectangleInventory {
        RectangleInventory { rectangles }
    }

    pub fn sort_by_width(&self, reverse: bool) -> (Vec<Rectangle>, u32) {
        self.sort_by_key(true, reverse)
    }

    pub fn sort_by_height(&self, reverse: bool) -> (Vec<Rectangle>, u32) {
        self.sort_by_key(false, reverse)
    }

    fn sort_by_key(&self, width: bool, reverse: bool) -> (Vec<Rectangle>, u32) {
        let mut list = self.rectangles.clone();
        let mut num_sort_operations = 0;

        if width {
            list.sort_by_key(|r| { num_sort_operations += 1;r.width })
        }
        {
            list.sort_by_key(|r| { num_sort_operations += 1;r.height })
        }

        if reverse {
            list.reverse();
        }

        (list, num_sort_operations)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Point<T, U: PartialOrd + Copy> {
    x: T,
    y: U,
}

// x >= y || x <= y // partial order

// in partial order, you can only sort some, but not all elements.
// BECAUSE some elements might be equal (==) to others elements

// x < y || x > y // total order
// in total ordering, you can sort all elements because every element is unique
// and has a unique and distinct value.

// constructor
impl<T, U: PartialOrd + Copy> Point<T, U> {
    pub(crate) fn new(x: T, y: U) -> Self {
        Point { x, y }
    }
}

impl<T, U: PartialOrd + Copy> Point<T, U> {
    pub(crate) fn x(self) -> T {
        self.x
    }

    pub(crate) fn y(self) -> U {
        self.y
    }

    pub(crate) fn mixup<V, W: PartialOrd + Copy>(self, other: Point<V, W>) -> Point<T, W> {
        return Point {
            x: self.x,
            y: other.y,
        };
    }
}

use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub(crate) enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug, Clone)]
pub(crate) enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Unknown
}

impl fmt::Display for Coin{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Coin::Penny => write!(f, "Penny"),
            Coin::Nickel => write!(f, "Nickel"),
            Coin::Dime => write!(f, "Dime"),
            Coin::Quarter(state) => write!(f, "Quarter from {:?}", state),
        }
    }
}

impl Coin {
    pub fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25,
        }
    }
}
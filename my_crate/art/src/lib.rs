//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    ///
    ///  # Example:
    /// ```
    ///  let red = PrimaryColor::Red;
    ///  let yellow = PrimaryColor::Yellow;
    ///  mix(red, yellow);
    /// ```
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        return SecondaryColor::Orange;
    }
}
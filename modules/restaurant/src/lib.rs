mod front_house;
mod back_house;

use crate::front_house::{hosting, serving};
use crate::back_house::{kitchen};

pub fn eat_at_restaurant() {

    hosting::seat_at_table();

    serving::take_order();

    kitchen::cook_order();

    serving::take_payments();
}



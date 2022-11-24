use crate::back_house::kitchen;
use crate::front_house::{hosting, serving};

mod front_house;
mod back_house;

pub fn eat_at_restaurant() {
    hosting::seat_at_table();

    serving::take_order();

    kitchen::cook_order();

    serving::take_payments();
}



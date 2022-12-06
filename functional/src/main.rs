use crate::closures::test_closures;
use crate::examples::{test_filter_shoes, test_inventory_shirts, test_sort_rectangle};
use crate::iterator::{test_int_iterator, test_iterator_adapters, test_string_iterator};

mod closures;
mod motivation;
mod iterator;
mod rectangle;
mod utils;
mod examples;
mod shirt_inventory;
mod shoe;

fn main() {
    // test_closures();
    // // iterate over primitive types
    // test_int_iterator();
    // test_string_iterator();
    // test_iterator_adapters();

    test_filter_shoes();
    test_sort_rectangle();
    test_inventory_shirts();
}

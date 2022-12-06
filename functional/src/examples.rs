use crate::shirt_inventory::ShirtColor;
use crate::utils::{get_rectangles_inventory, get_shirt_inventory, get_shoe_inventory, print_vector};

pub fn test_filter_shoes() {
    let shoe_inventory = get_shoe_inventory();

    let in_my_size = shoe_inventory.shoes_in_size(10);

    print_vector(&in_my_size);
}

pub fn test_sort_rectangle() {
    let inventory = get_rectangles_inventory();

    let (list, num_sort_operations) = inventory.sort_by_width(true);
    print_vector(&list);
    println!("Sorted by width in {num_sort_operations} operations");

    let (list, num_sort_operations) = inventory.sort_by_height(false);
    print_vector(&list);
    println!("Sorted by height in {num_sort_operations} operations");
}

pub fn test_inventory_shirts() {
    let mut inventory = get_shirt_inventory();

    let total_stock = inventory.remaining();
    println!("The initial stock is: {:#?}", total_stock);

    // Giveaway a shirt
    let user_pref = Some(ShirtColor::Red);
    let giveaway = inventory.give_away(user_pref);
    println!("The user with preference {:?} gets {:?}", user_pref, giveaway);

    let total_stock = inventory.remaining();
    println!("The initial stock is: {:#?}", total_stock);

    // Giveaway a shirt
    let user_pref = Some(ShirtColor::Blue);
    let giveaway = inventory.give_away(user_pref);
    println!("The user with preference {:?} gets {:?}", user_pref, giveaway);

    let total_stock = inventory.remaining();
    println!("The initial stock is: {:#?}", total_stock);
}

mod box_type;
mod my_box;
mod csp;

fn main() {
    box_type::test_box();
    box_type::test_cons_list();
    my_box::test_deref();
    my_box::test_deref_coercion();
    my_box::test_deref_my_box();

    println!();
    println!();
    csp::test_custom_smart_pointer();
}

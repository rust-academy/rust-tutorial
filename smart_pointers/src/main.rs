mod box_type;
mod my_box;
mod csp;
mod rc_type;
mod msg;
mod ref_cell_type;
mod tree;
mod mem_leak;

fn main() {
    println!("==test_box==");
    box_type::test_box();
    println!();

    println!("==test_cons_list==");
    box_type::test_cons_list();
    println!();

    println!("==test_deref==");
    my_box::test_deref();
    println!();

    println!("==test_deref_coercion==");
    my_box::test_deref_coercion();
    println!();

    println!("==test_deref_my_box==");
    my_box::test_deref_my_box();
    println!();

    println!("==test_custom_smart_pointer==");
    csp::test_custom_smart_pointer();
    println!();

    println!("==test_ref_count==");
    rc_type::test_ref_count();
    println!();

    println!("==test_mut_ref_cell==");
    ref_cell_type::test_mut_ref_cell();
    println!();

    println!("==test_mem_leak_cycle()==");
    mem_leak::test_mem_leak_cycle();
    println!();
}

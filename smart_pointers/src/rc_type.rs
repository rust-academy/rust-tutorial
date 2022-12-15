use std::rc::Rc;

use crate::rc_type::List::{Cons, Nil};

enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

// Doesn't work b/c Box cannot handle multiple owners
pub fn _test_mut_list_owners() {
    // let a = ConsList::Cons(5, Box::new(ConsList::Cons(10, Box::new(ConsList::Nil))));
    // let _b = ConsList::Cons(3, Box::new(a)); //  value a moved here
    // let c = ConsList::Cons(3, Box::new(a)); // Error: ^ value used here after move
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn test_ref_count() {
    // You have to enable multiple ownership explicitly by using the Rust type Rc<T>,
    // which is an abbreviation for reference counting.

    // The Rc<T> type keeps track of the number of references to a value to determine
    // whether or not the value is still in use.

    // If there are zero references to a value,
    // the value can be cleaned up without any references becoming invalid. (meaning memory will be cleaned up)

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Ref Count after creating a = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("Ref Count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Ref count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Ref count after c goes out of scope = {}", Rc::strong_count(&a));
} // b & a go out of scope so its memory will be cleaned

use std::cell::RefCell;
use std::rc::Rc;
use crate::mem_leak::RefList::{Cons, Nil};

#[derive(Debug)]
pub enum RefList{
    Cons(i32, RefCell<Rc<RefList>>),
    Nil
}

impl RefList {
    pub fn tail(&self) ->Option<&RefCell<Rc<RefList>>>{
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn test_mem_leak_cycle() {

    let a = Rc::new(
        Cons(
            5,
            RefCell::new(Rc::new(Nil))
        )
    );

    println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());


    let b = Rc::new(
        Cons(
            10,
            RefCell::new(Rc::clone(&a)),
        )
    );
    println!();

    println!("a rc count after creating b = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));

    // println!("b next item = {:?}", b.tail());
    println!();

    if let Some(link) = a.tail(){
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!();

    println!("a rc count after adding a link between a and b = {}", Rc::strong_count(&a));
    println!("b rc count after adding a link between a and b = {}", Rc::strong_count(&b));


    // Uncomment the next line to see that we have a cycle; it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // The reference count of the Rc<List> instances in both a and b are 2
    // after we change the list in a to point to b.
    //
    // At the end of main, Rust drops the variable b,
    // which decreases the reference count of the Rc<List> instance from 2 to 1.
    // The memory that Rc<List> has on the heap won’t be dropped at this point,
    // because its reference count is 1, not 0.
    // Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well.
    // This instance’s memory can’t be dropped either.
    // The memory allocated to the list will remain uncollected forever.
}

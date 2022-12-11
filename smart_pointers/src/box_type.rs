// Boxes don’t have performance overhead, other than storing their data on the heap.
//  You’ll use them most often in these situations:
//
// * When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// * When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// * When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

pub fn test_box() {
    let b = Box::new(5);
    println!("{}", b);
}

pub fn test_deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

enum SimpleList {
    // Cons(i32, SimpleList),
    Nil,
}

#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}

pub fn test_cons_list(){
    //let list = SimpleList::Cons(1, SimpleList::Cons(2, SimpleList::Nil));

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    println!("{:?}", list);
}
// Boxes don’t have performance overhead, other than storing their data on the heap.
// You’ll use them most often in these situations:
//
// 1) When you have a type whose size can’t be known at compile time and you want to use
//    a value of that type in a context that requires an exact size. (meaning code with dynamically sized data structures to compile)
//
// 2) When you have a large amount of data and you want to transfer ownership,
//    but ensure the data won’t be copied when you do so.
//
// 3) When you want to own a value and you care only that it’s a type that
//    implements a particular trait rather than being of a specific type. (meaning, you want to assume ownership of data)
//
// * Recursive data structures

pub fn test_box() {
    let b = Box::new(5);
    println!("Box contains: {}", b); // Rust deref a box automatically.
}

// enum SimpleList{
//     Cons(i32, SimpleList), // item in list with another list in a list
//     Nil, // No item in list
// }

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn test_cons_list() {
    // doesn't work :-(
    // the problem is that the compiler cannot determine the memory size
    // of an infinite recursive constructor call
    // let list = SimpleList::Cons(1, SimpleList::Cons(2, SimpleList::Nil));

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

    println!("{:?}", list);
}




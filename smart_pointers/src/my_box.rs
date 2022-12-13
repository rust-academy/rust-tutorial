use std::ops::Deref;

pub fn test_deref() {
    let x = 5;
    let y = Box::new(5);

    assert_eq!(x, 5);
    assert_eq!(*y, 5); // we can deref a box and get the pointed value from it.
    // Note though, box points at a *copied* value of x so there is no actual memory gain.

    println!("Everything okay");
}

pub fn test_deref_my_box() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("All checks passed! ")
}


fn hello(name: &str) {
    println!("Hello: {name}")
}

pub fn test_deref_coercion() {
    let m = MyBox::new(String::from("Rust"));

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    //
    // * From &T to &U when T: Deref<Target=U>
    //
    // * From &mut T to &mut U when T: DerefMut<Target=U>
    //
    // * From &mut T to &U when T: Deref<Target=U>


    // The first two cases are the same as each other except that the second implements mutability.
    // The first case states that if you have a &T, and T implements Deref to some type U,
    // you can get a &U transparently. (reads automatically)
    //
    // The second case states that the same deref coercion happens for mutable references.
    //
    // The third case is an interesting one: Rust will also coerce a mutable reference to an immutable one.
    // But the reverse is not possible: immutable references will never coerce to mutable references.
    // Because of the borrowing rules, if you have one mutable reference,
    // that one mutable reference must be the one and only reference to that data.

    // Rust coerceives &String into a &str
    hello(&m); //
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

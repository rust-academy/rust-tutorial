use std::thread;
use std::time::Duration;

pub fn test_closures() {
    simple_closure();
    closure_syntax();
    closure_context_capture();
    closure_context_value();
}

fn simple_closure() {
    println!("simple_closure");

    let _x = 5;

    let expensive_closure = |num: i32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_micros(num as u64));
        num + 21
    };

    let forty_two = expensive_closure(21);
    println!("{}", forty_two)
}

fn closure_syntax() {
    println!("closure_syntax");
    // Closure syntax:
    // * No function name required. Similar to an anonymous inner function.
    // * No argument declaration required; a closure captures its argument implicitly hence no need to declare.
    // * No type annotation required; optional argument type to improve type inference
    // * No return type declaration required; the last expression is stored in the assigned value.
    // * Curly brackets are optional; handy for concise expression of short expressions i.e. x+1
    // * Double pipe annotation "||" is optional when return type is void
    // * Semicolon required ;-)

    // Most simple_closure syntax
    let add_one = |num: i32| num + 1;
    println!("{}", add_one(0));

    // Overblown syntax with all clutter
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    ;

    println!("{}", add_one_v1(1));

    // Removed argument name and type. Type inference comes from annotated captured value
    let add_one_v2 = |x: u32| -> u32 { x + 1 };

    println!("{}", add_one_v2(2));

    // remove all type annotation
    let add_one_v3 = |x| { x + 1 };
    println!("{:?}", add_one_v3(3)); // println falls back to debug b/c of missing type inference

    // Removed curly brackets.
    let add_one_v4 = |x| x + 1;
    println!("{:?}", add_one_v4(4));

    // Removed brackets, but added return type to restore call site type inference.
    let add_one_v5 = |x: i32| x + 1;
    println!("{}", add_one_v5(5));

    // store closure in a value
    let one_plus = add_one_v5(5);

    // pass around that value
    println!("{}", one_plus);

    let no_arg = || println!("Nothing captured");
    no_arg();
}

fn closure_context_capture() {
    // Closures capture values from their local context in one of three ways:

    // 1) borrowing immutably.
    immutable_borrow_closure();

    // 2) borrowing mutably.
    mutable_borrow_closure();

    // 3) move and taking ownership.
    taking_ownership_closure();
}

fn immutable_borrow_closure() {
    println!("immutable_borrow_closure");

    // collection with sample data
    let list = vec![1, 2, 3];

    let only_borrow = || println!("From closure: {:?}", list); // create immutable reference to list

    println!("Before calling closure: {:?}", list);
    only_borrow();
    println!("After calling closure: {:?}", list);
}

fn mutable_borrow_closure() {
    println!("mutable_borrow_closure");
    // mutable collection with sample data
    let mut list = vec![1, 2, 3];

    println!("Before calling closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // this is the only place where you cannot add any further reference without getting an error
    borrows_mutably();
    // here it is safe to reference again
    println!("After calling closure: {:?}", list);
}


fn taking_ownership_closure() {
    println!("taking_ownership_closure");

    // If you want to force the closure to take ownership of the values it uses in the environment
    // even though the body of the closure doesn’t strictly need ownership,
    // you can use the move keyword before the parameter list.

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let closure = println!("From thread: {:?}", list);

    thread::spawn(move || closure)
        .join()
        .unwrap();
}


fn closure_context_value() {
    //  A closure body can do any of the following to the value captured fro its context:

    // 1) move a captured value out of the closure,
    move_captured_value_closure();

    // 2) mutate the captured value,
    mutate_captured_value_closure();

    // 3) neither move nor mutate the value i.e. read only
    no_ops_closure();

    // 4) or capture nothing from the environment to begin with.
    capture_nothing_closure();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn move_captured_value_closure() {
    println!("move_captured_value_closure");

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations} operations", list)
}


fn mutate_captured_value_closure() {
    println!("mutate_captured_value_closure");

    let mut list = vec![1, 2, 3];

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("{:?}", list)
}


fn no_ops_closure() {
    println!("no_ops_closure");
    // define a mutable value within scope
    let num_sort_operations = 42;

    {
        let num_sort_operations = 23;
        let print_ops = || println!("Sorting took nr. ops {}", num_sort_operations);
        print_ops();
    }

    let print_ops = || println!("Sorting took nr. ops {}", num_sort_operations);
    print_ops();
}


fn capture_nothing_closure() {
    println!("capture_nothing_closure");

    let only_print = || println!("hi");
    only_print();
}

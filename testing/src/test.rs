use super::*;

#[test]
fn test_add() {
    assert_eq!(add(2, 2), 4);
}

#[test]
fn test_add_two_eq() {
    assert_eq!(add_two(2), 4);
    assert_eq!(add_two(1), 3);
    assert_eq!(add_two(2), 4);
    assert_eq!(add_two(4), 6)
}

#[test]
fn test_add_two_not_eq() {
    assert_ne!(add_two(2), 1);
    assert_ne!(add_two(2), 3);
    assert_ne!(add_two(2), 5);
}


#[test]
fn test_greetings() {
    let result = greetings("Carol");
    assert_eq!(result, "Hello, Carol!");
}


#[test]
fn test_greetings_contains() {
    let name = "Carol";
    let result = greetings(name);
    assert_eq!(result.contains(name), true);
}

#[test]
fn test_larger_can_hold_smaller() {
    let (smaller, larger) = test_utils::get_small_and_large_rectangle();
    assert_eq!(larger.can_hold(&smaller), true)
}

#[test]
fn test_smaller_cannot_hold_larger() {
    let (smaller, larger) = test_utils::get_small_and_large_rectangle();
    assert_ne!(smaller.can_hold(&larger), true)
}

#[test]
fn test_new_valid_guess() {
    assert_eq!(Guess::new(1).value(), 1);
    assert_eq!(Guess::new(2).value(), 2);
    assert_eq!(Guess::new(3).value(), 3);
}

#[test]
#[should_panic(expect = "Guess value must be greater or equal to 1")]
fn test_new_invalid_guess_below_1() {
    let _g = Guess::new(-1);
}

#[test]
#[should_panic(expect = "Guess value must less than or equal to 100")]
fn test_new_invalid_guess_above_100() {
    let _g = Guess::new(101);
}


pub mod test_utils {
    use super::*;

    pub fn get_small_and_large_rectangle() -> (Rectangle, Rectangle) {
        let smaller = Rectangle { width: 2, height: 2 };
        let larger = Rectangle { width: 8, height: 7 };
        return (smaller, larger);
    }
}



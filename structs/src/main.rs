#[derive(Debug, Clone, PartialEq)]
struct User {
    username: String,
    email: String,
    counter: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User { username, email, counter: 0, active: true }
}

#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn build_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.height > other.height && self.width > other.width;
    }
}


fn main() {
    println!("Hello, structs!");

    let user1 = User {
        username: "Sam".to_string(),
        email: "sam@mail.com".to_string(),
        counter: 0,
        active: true,
    };
    let user2 = build_user(String::from("Lisa"), String::from("lise@mail.com"));

    let user3 = User {
        username: String::from("SamLOL"),
        ..user1
    };

    println!("{:#?}", user3);

    let rect = Rectangle { width: 30, height: 50 };
    println!("{:#?}", rect);

    println!(
        "The area of the rectangle is: {}",
        rect.area()
    );

    let rect1 = Rectangle { width: 20, height: 40 };
    let rect2 = Rectangle { width: 40, height: 50 };

    println!("rect can hold rect1 {} ", rect.can_hold(&rect1));
    println!("rect can hold rect2 {} ", rect.can_hold(&rect2));

    let r3 = Rectangle::build_square(32);
    let s = String::from("Something");

    #[derive(Debug, Clone, PartialEq)]
    struct Color(i32, i32, i32);
    struct Point(i32, i32);

    let red = Color(1, 0, 0);

    let r = red.0;

    let point = Point(23, 42);

    let x = point.0;
}

#[derive(Debug)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    } 
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height;
// }

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("walter123"),
        email: String::from("walter@example.com"),
        sign_in_count: 1,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("alex123");

    let user2 = build_user(
        String::from("alex@example.com"), 
        String::from('alex123')
    );

    let user3 = User {
        username: String::from("walter123"),
        email: String::from("walter@example.com"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let width1 = 20;
    let height1 = 20;

    let rect = Rectangle {
        width: 30,
        height: 50
    };
    println!("rect: {:#?}", rect);
    
    // println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(25);
}
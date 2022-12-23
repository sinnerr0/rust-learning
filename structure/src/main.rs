fn main() {
    print_user();
    print_rectangle();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user() {
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("mut user1 email is {}", user1.email);

    let user2 = build_user(String::from("email"), String::from("username"));
    println!("user2 email is {}", user2.email);

    let user3 = User {
        email: String::from("user3email@example.com"),
        ..user2
    };
    println!("user3 email is {}", user3.email);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn print_rectangle() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("사각형의 면적: {} 제곱 픽셀", rect1.area());
    println!("rect1 {:#?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));

    println!("rect square {:?}", Rectangle::square(10));
}

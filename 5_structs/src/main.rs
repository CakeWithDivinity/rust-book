struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.area() > rectangle.area()
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("FooBar42"),
        email: String::from("FooBar42@foo.bar"),
        sign_in_count: 1
    };

    user1.email = String::from("changedEmail@foo.bar");

    let user2 = User {
        email: String::from("asdf@asdf.asdf"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let origin_y = origin.0;

    let subject = AlwaysEqual;
    
    let rectangle = Rectangle {
        width: 30, 
        height: 50,
    };
    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rectangle3 = Rectangle {
        width: 60,
        height: 45,
    };


    println!("The area of the rectangle is {}", calculate_area(&rectangle));
    println!("The area of the rectangle is {}", rectangle.area());
    println!("rectangle is {:#?}", rectangle);
    dbg!(&rectangle);

    println!("Can rectangle1 hold rectangle2? {}", rectangle.can_hold(&rectangle2));
    println!("Can rectangle1 hold rectangle3? {}", rectangle.can_hold(&rectangle3));

    let square = Rectangle::square(20);
    println!("The area of the square is {}", square.area());
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

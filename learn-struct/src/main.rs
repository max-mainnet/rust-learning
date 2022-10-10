impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct User0 {
//     username: &str, // error[E0106]: missing lifetime specifier
// }
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

// tuple struct
struct RectangleTupleStruct(u32, u32);

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}

fn main() {
    println!("Hello, world!");

    // mut keyword makes variable mutable for all members
    let user1 = User {
        email: String::from("max@ref.finance"),
        username: String::from("max"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user("max@ref.finance".to_string(), "max".to_string());

    println!("user1Name: {}", user1.username);

    let user3 = User {
        email: String::from("willa@ref.finance"),
        username: String::from("willa"),
        ..user2
    };

    println!(
        "{}, {}, {}, {}",
        user3.username, user3.email, user3.active, user3.sign_in_count
    );

    let rectangle0 = Rectangle {
        height: 50,
        width: 30,
    };

    println!("area: {}", rectangle0.area());

    let rectangle1 = RectangleTupleStruct(20, 30);

    println!("area: {}", area_tuple((rectangle1.0, rectangle1.1)));

    println!("{:#?}", rectangle0);

    let rec1 = Rectangle {
        height: 40,
        width: 40,
    };

    let rec2 = Rectangle {
        height: 20,
        width: 10,
    };

    println!("can rec1 hold rec2: {}", rec1.can_hold(&rec2));

    println!("can rec2 hold rec1: {}", rec2.can_hold(&rec1));

    let squre0 = Rectangle::square(rec1.height);

    println!("squre0: {:#?}", squre0);
}

fn area_tuple(dim: (u32, u32)) -> u32 {
    return dim.0 * dim.1;
}

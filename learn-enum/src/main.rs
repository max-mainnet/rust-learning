enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        }
    }
}

fn main() {
    println!("Hello, world!");

    let addr = IpAddrKind::V4(1, 2, 3, 4);

    let v6 = IpAddrKind::V6(String::from("::1"));

    let quit = Message::Quit;

    let message = Message::Move { x: 1, y: 2 };

    let write = Message::Write(String::from("hello"));

    let c = Message::ChangeColor(1, 2, 3);

    let some_number = Some(5);

    let some_string = Some("a string");

    let absent_number: Option<u32> = None;

    let x = 8;
    let y = Some(5);

    println!("x = {:?}, y = {:?}", x, y.unwrap());

    message.call();

    println!("value in cents: {}", value_in_cents(Coin::Penny));

    let c = Coin::Quarter(UsState::Alaska);

    println!("value in cents: {}", value_in_cents(c));

    let ot = Some(5);

    let res = plus_one(ot);

    println!("res = {:?}", res);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // must iterate over the Option<T> by using match
    // match x {
    //     // None => None, //error ^ pattern `None` not covered
    //     Some(i) => Some(i + 1),
    //     _ => None,
    // }
    if let Some(i) = x {
        Some(i + 1)
    } else {
        None
    }
}

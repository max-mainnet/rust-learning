const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Hello, world!");

    let x = 5;

    // shadowing
    let x = x + 1;

    println!("x = {}", x);

    let x = x * 2;

    let spaces = "   ";

    let spaces = spaces.len();

    println!("spaces = {}", spaces);

    println!("x = {}", x);

    println!("MAX_POINTS = {}", MAX_POINTS);

    // giving expect number
    let guess: i32 = "42".parse().expect("Not a number!");

    println!("guess = {}", guess);

    let _x = 2.0; // f64 default

    let _y: f32 = 3.0; // f32

    let _sum = 5 + 10;

    let _difference = 95.5 - 4.3;

    let _true: bool = true;

    let z = '😂';

    println!("z = {}", z);

    // could be different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    // destructure
    println!("tup = {:?}", tup);

    println!("x = {}", x);

    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let index = [13, 14];

    let first = _months[0];

    println!("first = {}", first);

    // index out of bounds
    println!("panic check {}", _months[index[0]]);
}

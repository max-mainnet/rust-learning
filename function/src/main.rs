fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    // let y = {
    //     let mut x = 3;
    //     x + 3; // ()
    // };

    println!("The value of y is: {}", five(5));

    // println!("The value of y is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five(x: i32) -> i32 {
    return x + 6;
}

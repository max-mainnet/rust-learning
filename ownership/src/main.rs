fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", s);

    let x = 5;

    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");

    // let s2 = s1; //stack copy, heap move

    // println!("{}, world!", s1); // error: value borrowed here after move

    // println!("{}, world!", s2);

    let s2 = s1.clone(); // stack copy, heap copy

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let z = 5;

    let s = String::from("Hello World");

    take_owner(s);

    // println!("{}", s); // error: value borrowed here after move

    makes_copy(z);

    println!("{}", z);

    let s1 = gives_ownership();

    let s2 = String::from("Hello World");

    // s2 loses ownership, s3 takes ownership from s2
    let s3 = takes_and_gives_back(s2);

    println!("{}", s1); // s1 takes ownership from gives_ownership() function;

    // println!("{}", s2); // error: value borrowed here after move

    println!("{}", s3); // s3 takes ownership from takes_and_gives_back() function;

    let s3_len = calculate_only_length(&s3);

    println!("The length of '{}' is {}.", s3, s3_len);

    let (s4, len) = calculate_length(s3);

    println!("The length of '{}' is {}.", s4, len);

    let mut s5 = String::from("hello");

    println!(
        "The length of s5 is {} {} {} .",
        s5.len(),
        calculate_only_length_mutable(&mut s5),
        s5.len()
    );

    let mut s6 = String::from("hello");

    // let s8 = &mut s6; // error: second mutable borrow occurs here

    {
        // let s8 = &mut s6; // ok

        // println!("s8 is {}.", s8);

        println!("s6 is {}.", s6);
    }

    let s7 = &mut s6;

    // println!("s6 is: {}", s6); // error: immutable borrow occurs here, mutable borrow later used here

    // s7.push_str(", world");

    println!("s7: {}", s7);

    // println!("{}", s3);// error: value borrowed here after move
}
// free, drop s, drop d2, drop s1, drop s3, drop s4, drop s5, drop s6;

fn gives_ownership() -> String {
    let some_string = String::from("Hello World");

    return some_string;

    // drop some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

fn take_owner(some_string: String) {
    // some_string into scope
    println!("{}", some_string);

    // drop some_string; heap free;
}

fn makes_copy(some_integer: i32) {
    // some_integer into scope
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

// &s is a reference to s, don't get ownership
fn calculate_only_length(s: &String) -> usize {
    // s, borrowed, immutable
    // s.push_str(", world"); // error: cannot borrow as mutable
    s.len()
}

fn calculate_only_length_mutable(s: &mut String) -> usize {
    // s, borrowed, immutable
    s.push_str(", world");
    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // drop s, but return reference to s, error: missing lifetime specifier

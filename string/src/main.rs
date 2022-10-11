fn main() {
    println!("Hello, world!");

    let data = "initial contents";

    let mut s1 = data.to_string();

    let mut s = String::from(data);

    let s0 = String::new(); // empty string

    s.push_str(&s1);

    s.push('😊');

    println!("{}", s1);

    println!("{}", s);

    println!("{}", s + &s1);

    let s1 = String::from("Hello");
    let s2 = String::from("world");

    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used

    let s3 = format!("{}{}", s2, s3);

    println!("{}", s3);

    println!("{}", s2);

    // println!("{}", s1); // error

    println!("{}", s2.len());

    let w = "नमस्ते";

    for b in w.bytes() {
        println!("{}", b);
    }

    for b in w.chars() {
        println!("{}", b);
    }

    let hello = "Здравствуйте";

    // let s = &hello[..3]; //byte index 3 is not a char boundary

    // println!("{}", s);

    let s0 = String::from("hello");

    let s2 = &s0[0..2];
}

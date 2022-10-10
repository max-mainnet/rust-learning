fn main() {
    // let s0 = "Hello World";

    let s = String::from("Hello world");
    let first_word = first_word(&s[..]);
    // s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("word index: {}", word_index);

    // println!("the first_word is {}", &s[..word_index]);
    println!("the first_word is {}", first_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

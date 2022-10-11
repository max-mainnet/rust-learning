use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("hello"), 10);

    scores.insert(String::from("yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];

    let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    let mut map = HashMap::new();

    let field_name = String::from("Favorite color");

    let field_color = String::from("Blue");

    map.insert(&field_name, &field_color);

    println!("{},{}", field_color, field_name); // value borrowed here after move

    let color = map.get(&field_name);

    match color {
        Some(&value) => println!("{}", value),
        None => println!("None"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 10);

    println!("{:?}", scores);

    scores.insert(String::from("Blue"), 20);

    println!("{:?}", scores);

    scores.entry(String::from("yellow")).or_insert(50);

    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

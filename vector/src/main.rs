enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    // let v1 = vec![1, 2, 3];

    v.push(1);

    v.push(2);

    v.push(3);

    println!("{:?}", v);

    println!("Hello, world!");

    let v2 = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v2[100]; // index is out of bounds

    // println!("The third element is {}", third);

    match v.get(100) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(2); // error: cannot borrow `v` as mutable because it is also borrowed as immutable

    println!("The first element is: {}", first);
    println!("{:?}", v);

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

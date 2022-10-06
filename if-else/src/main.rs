fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let mut counter = 0;
    let mut res = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    while res != 0 {
        println!("{}!", res);
        res = res - 1;
    }

    println!("The value of res is: {}", res);

    let a = [1, 23, 4, 5, 5];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }

    // iterating over an array
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // iterrated over a range

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

use std::io;

use rand::Rng; // trait

use std::cmp::Ordering;

fn main() {
    println!("guessing !");

    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101); // i32 u32 i64

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

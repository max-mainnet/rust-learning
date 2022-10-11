// panic!

use std::io::{self, Read};

// use core::panic;
use std::fs::File;

use std::net::IpAddr;

use rand::Rng;

use std::cmp::Ordering;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        return self.value;
    }
}

fn main() {
    println!("Hello, world!");
    // panic!("crash and abort");

    // let f = File::open("hello.txt");

    // let _f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         _ => panic!("There was a problem opening the file: {:?}", error),
    //     },
    // };

    let _f0 = File::open("hello.txt").expect("Failed to open hello.txt");
    // v[99];

    let result = read_username_from_file();

    println!("{}", result.unwrap());

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    let random_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = Guess::new(1);

    loop {
        let mut guessValue = String::new();

        io::stdin()
            .read_line(&mut guessValue)
            .expect("Failed to read line");

        let myGuess = Guess::new(guessValue.trim().parse().expect("Please type a number!"));

        match myGuess.value().to_string().cmp(&random_number.to_string()) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    // let mut a = match a {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // a.read_to_string(&mut s)?;

    return Ok(s);

    // match a.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}

use std::collections::HashMap;

use std::fmt;
use std::io;

// fn f1() -> fmt::Result {}

// fn f2() -> io::Result {}

mod front_of_house;
pub fn eat_at_restuarant_pre() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // all members are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub use crate::front_of_house::hosting;

use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restuarant() {
    let mut meal = back_of_house::Breakfast::summer("Tye");

    meal.toast = String::from("Wheat");

    add_to_waitlist();

    // hosting::some_functon(); // error : function is private
    // public member could be accessed
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // error private member
}

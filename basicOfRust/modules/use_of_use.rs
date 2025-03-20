use std::collections::HashMap;
use rand::Rng;

//rand = "0.9.0"

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    add_to_waitlist();
}

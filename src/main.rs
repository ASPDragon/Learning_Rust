use crate::structs_enums::structEnums::Coin;
use crate::structs_enums::structEnums::UsState;
use std::collections::HashMap;

mod guessing_game;
mod structs_enums;

mod front_of_house;

use crate::front_of_house::hosting;

use crate::structs_enums::structEnums;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

fn main() {
    println!("Task one:");
    let mut v = vec![ 1, 13, 5, 100, 13, 35, 42 ];
    v.sort();
    let med: i32 = v[v.len() / 2];
    println!("The median is: {med}");
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    // Find the key/value pair with the maximum value
    let max_pair = map.iter().max_by_key(|entry| entry.1);

    if let Some((key, value)) = max_pair {
        println!("Key with max value: {key}, Value: {value}");
    } else {
        println!("The map is empty.");
    }
    println!("\nTask two:");
}

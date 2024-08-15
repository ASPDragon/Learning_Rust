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
    println!("{}", structEnums::value_in_cents(Coin::Quarter(UsState::Iowa)));
    let five = Some(5);
    let six = structEnums::plus_one(five);
    println!("{}", six.unwrap());

    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut map = HashMap::new();
    map.insert(1, 2);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let mut s = String::new();

    let mut s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    s1 += &s2;

    println!("{s1}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{s}");

    for c in hello.chars() {
        println!("{c}");
    }
}

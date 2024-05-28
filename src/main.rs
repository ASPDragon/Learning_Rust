use crate::structs_enums::structEnums::Coin::Quarter;
use crate::structs_enums::structEnums::UsState::Iowa;

mod guessing_game;
mod structs_enums;

use crate::structs_enums::structEnums;

fn main() {
    println!("{}", structEnums::value_in_cents(Quarter(Iowa)));
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
}

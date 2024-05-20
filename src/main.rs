mod guessing_game;

use crate::Coin::{Penny, Quarter};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(dimension: u32) -> Self {
        Self {
            width: dimension,
            height: dimension,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, rhs: &Rectangle) -> bool {
        (self.width > rhs.width && self.height > rhs.height) ||
            (self.height > rhs.width && self.width > rhs.height)
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // blah blah
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    println!("{}", value_in_cents(Quarter(UsState::Iowa)));
    guessing_game();
}

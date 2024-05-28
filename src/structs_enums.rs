pub(crate) mod structEnums {
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
    pub enum UsState {
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

    pub enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    pub fn value_in_cents(coin: Coin) -> u8 {
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

    fn count_coins_number(coin: Coin) -> u8 {
        // let mut count = 0;
        // match coin {
        //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        //     _ => count += 1,
        // }

        let mut count = 0;
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
        count
    }

    pub fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
}
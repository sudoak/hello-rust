use crate::Coin;

pub fn value_in_cents(coin: & Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::States(state) => {
            println!("{:?}",state);
            0
        }
    }
}
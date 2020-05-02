use ferris_says::say;
use std::io::{stdout, BufWriter, stdin};
use std::sync::mpsc;
use std::thread;
use std::sync::Mutex;

// modules
mod rectangle;
mod borrow_string;
mod guess_number;
mod enum_coin;
mod some_example;
mod module_exmple;
mod collection_example;
mod error_example;
mod guess_number_struct;
mod greatest_of_all;
mod trait_sample;
mod cons_list_example;
mod thread_example;

// use imports 
use guess_number::guess_number;
use rectangle::area;
use enum_coin::value_in_cents;
use some_example::add_one;
use module_exmple::eating_at_cafe;
use collection_example::{ get_hash_map_color, has_map_from_vec, count_words_in_a_sentence };
use error_example::read_file_here;
use guess_number_struct::{ Guess};
use greatest_of_all::largest;
use trait_sample::run;
use cons_list_example::print_list;
use thread_example::thread_main;

// structs & enums
#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
pub enum UStates {
    Alaska,
    Alabama
}

pub enum Coin {
    Penny,
    Nickel, 
    Dime, 
    Quarter,
    States(UStates)
}

// implementations of struct and enums
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
    fn square(side: u32) -> Rectangle {
        Rectangle { width : side, height: side}
    }
}

// Main 
fn main() {
    let stdout = stdout();
    let out = b"Program runs Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
    
    println!("Please enter a choice");

    let mut input_option = String::new();
    stdin().read_line(&mut input_option)
            .expect("Failed to read number");
    let input_option: u32 = match input_option.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input option"),
    };
    match input_option {
        1 => guess_number(),
        2 => area(),
        3 => {
            let first_coin = Coin::Penny;
            let state_be = Coin::States(UStates::Alabama);
            println!("value be {}",value_in_cents(&first_coin));
            println!("state be {}",value_in_cents(&state_be));
        },
        4 => {
            let sample_input = Some(1);
            println!("Solution is {:?}", add_one(&sample_input))
        },
        5 => eating_at_cafe(),
        6 => get_hash_map_color(),
        7 => has_map_from_vec(),
        8 => count_words_in_a_sentence(),
        9 => read_file_here(),
        10 => {
            let guess_one = Guess::new(10);
            println!("{:?}", guess_one.value());
        },
        11 => {
            let number_list = vec![34, 50, 25, 100, 65];
            let result = largest(&number_list);
            println!("The largest number is {}", result);
        },
        12 => run(),
        13 => {
            print_list();
        },
        14 => thread_main(),
        15 => {
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();
            });
            let received = rx.recv().unwrap();
            println!("Got: {}", received);
        },
        16 => {
            let m = Mutex::new(5);
            {
                let mut num = m.lock().unwrap();
                *num = 6;
            }
            println!("m = {:?}", m);
        },
        _ => println!("You entered wrong input")
    }
}
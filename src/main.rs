use ferris_says::say;
use std::io::{stdout, BufWriter, stdin};

mod rectangle;
mod borrow_string;

mod guess_number;
use guess_number::guess_number;
use rectangle::area;

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

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
        _ => println!("You entered wrong input")
    }
}
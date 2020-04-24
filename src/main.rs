use ferris_says::say;
use std::io::{stdout, BufWriter};

mod rectangle;
mod borrow_string;

mod guess_number;
use guess_number::guess_number;
use rectangle::area;

pub struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let stdout = stdout();
    let out = b"Program runs Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
    
    guess_number();

    // Calculate area of rectangle
    let first_rectangle = Rectangle { width: 10, height: 10};
    println!("The area of rectangle is: {}", area(&first_rectangle));
}
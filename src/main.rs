use ferris_says::say;
use std::io::{stdout, BufWriter};

mod rectangle;
mod borrow_string;

mod guess_number;
// use guess_number::guess_number;
// use rectangle::area;

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let stdout = stdout();
    let out = b"Program runs Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
    
    // guess_number();

    // Calculate area of rectangle
    let first_rectangle = Rectangle { width: 10, height: 10};
    println!("The traiangle has {:#?}", first_rectangle);
    println!("The area of rectangle is: {}", first_rectangle.area());
}
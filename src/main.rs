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
    
    // guess_number();

    // Calculate area of rectangle
    let first_rectangle = Rectangle { width: 10, height: 10};
    let second_rectangle = Rectangle { width: 1, height: 5};
    let first_square = Rectangle::square(32);
    println!("The traiangle has {:?}", first_rectangle);
    println!("The area of first rectangle is: {}", first_rectangle.area());
    println!("can first rectanlge hold second one {}", first_rectangle.can_hold(&second_rectangle));
    println!("The square is {:0?} and its area is {1}", first_square, first_square.area());
}
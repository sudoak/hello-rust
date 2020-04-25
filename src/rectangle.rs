use crate::Rectangle;

pub fn area() {
    // Calculate area of rectangle
    let first_rectangle = Rectangle { width: 10, height: 10};
    let second_rectangle = Rectangle { width: 1, height: 5};
    let first_square = Rectangle::square(32);
    println!("The traiangle has {:?}", first_rectangle);
    println!("The area of first rectangle is: {}", first_rectangle.area());
    println!("can first rectanlge hold second one {}", first_rectangle.can_hold(&second_rectangle));
    println!("The square is {:0?} and its area is {1}", first_square, first_square.area());
}
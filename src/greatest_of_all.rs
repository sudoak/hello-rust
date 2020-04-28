use std::cmp::PartialEq;

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//     U: Clone + Debug
// {

// }


// generic types

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// enum Option<T> {
//     Some(T),
//     None,
// }

// struct Point<T, U> {x: T, y: U,}
// impl<T, U>Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
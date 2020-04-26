// use std::cmp::PartialOrd;

// pub fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
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
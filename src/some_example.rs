
pub fn add_one(number: &Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(num) => Some(num + 1 )
    }
}
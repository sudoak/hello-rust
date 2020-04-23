use rand::{thread_rng, Rng};
use std::io::{stdin};
pub fn guess_number(){
    let mut rng = thread_rng();
    let secret_number: u32 = rng.gen_range(0, 10);
    println!("Please guess a number");
    let mut guessed_number = String::new();
    stdin().read_line(&mut guessed_number)
        .expect("Failed to read number");
    
    println!("Guessed Number: {} Secret Number: {}", guessed_number, secret_number);
}
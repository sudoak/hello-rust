use rand::{thread_rng, Rng};
use std::io::{stdin};
use std::cmp::Ordering;

pub fn guess_number(){
    let mut rng = thread_rng();
    loop {
        let secret_number = rng.gen_range(0, 10);
        println!("Please guess a number");
        let mut guessed_number = String::new();
        stdin().read_line(&mut guessed_number)
            .expect("Failed to read number");
        
        let guess: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("Hurray its equal");
                break;
            }
        }
    }
}
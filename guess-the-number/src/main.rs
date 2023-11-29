use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to Guess the Number!");
    println!("I'm thinking of a number between 1 and 10.");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("That's too low, try again: "),
            Ordering::Greater => println!("That's too high, try again:"),
            Ordering::Equal => {
                println!("You got it! {guess} was the answer 👑");
                break;
            }
        }
    }
}

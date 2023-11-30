use rand::Rng;
use std::{cmp::Ordering, io};

const MAX_SECRET_NUMBER: u8 = 10;
const AMOUNT_OF_TRIES: u8 = 3;

fn main() {
    println!("Welcome to Guess the Number!");
    println!("I'm thinking of a number between 1 and 10.");

    let secret_number = rand::thread_rng().gen_range(1..=MAX_SECRET_NUMBER);

    for attempt in 1..=AMOUNT_OF_TRIES {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You got it! {guess} was the answer 👑");
                break;
            }
            Ordering::Greater | Ordering::Less if attempt == AMOUNT_OF_TRIES => {
                println!("You ran out of tries!\n{secret_number} was the answer!");
                break;
            }
            Ordering::Less => println!("That's too low, try again: "),
            Ordering::Greater => println!("That's too high, try again:"),
        }
    }
}

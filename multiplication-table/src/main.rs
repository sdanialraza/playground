use prettytable::{Cell, Row, Table};
use std::io::stdin;

fn main() {
    println!("Please enter a number for the multiplication table (1-255):");

    let mut number = String::new();

    stdin()
        .read_line(&mut number)
        .expect("Failed to read the number");

    let number: u16 = match number.trim().parse() {
        Ok(num) if num <= 255 => num,
        _ => {
            eprintln!("Invalid input, please enter a valid number");
            return;
        }
    };

    println!("Please enter a limit for the multiplication table (1-255):");

    let mut limit = String::new();

    stdin()
        .read_line(&mut limit)
        .expect("Failed to read the limit");

    let limit: u16 = match limit.trim().parse() {
        Ok(num) if num <= 255 => num,
        _ => {
            eprintln!("Invalid input, please enter a valid limit");
            return;
        }
    };

    let mut table = Table::new();

    for multiplier in 1..=limit {
        let product = number * multiplier;

        table.add_row(Row::new(vec![
            Cell::new(&format!("{} x {}", number, multiplier)),
            Cell::new(&format!("{}", product)),
        ]));
    }

    table.printstd();
}

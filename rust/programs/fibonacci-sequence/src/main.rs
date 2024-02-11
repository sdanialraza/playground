use std::io::stdin;

fn main() {
    println!("Please enter a limit for the Fibonacci Sequence");

    let mut limit = String::new();

    stdin()
        .read_line(&mut limit)
        .expect("Failed to read the number");

    let limit: u8 = match limit.trim().parse() {
        Ok(limit) if limit < 128 => limit,
        _ => {
            eprintln!("The limit must be less than 128");
            return;
        }
    };

    let sequence = fibonacci_sequence(limit);
    let sequence_size = fibonacci_sequence_size(limit);

    println!("{:?}", sequence);
    println!("{:?}", sequence_size)
}

fn fibonacci_sequence(limit: u8) -> Vec<u128> {
    let mut sequence: Vec<u128> = Vec::new();

    let mut first = 0;
    let mut second = 1;

    while first <= limit.into() {
        sequence.push(first);

        let next = first + second;

        first = second;
        second = next;
    }

    sequence
}

fn fibonacci_sequence_size(length: u8) -> Vec<u128> {
    let mut sequence = Vec::new();

    let mut first = 0;
    let mut second = 1;

    for _ in 0..length {
        sequence.push(first);

        let next = first + second;

        first = second;
        second = next
    }

    sequence
}

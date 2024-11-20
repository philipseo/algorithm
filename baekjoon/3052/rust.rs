use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    let mut numbers = HashSet::new();

    for _ in 0..10 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let number: u16 = input.trim().parse().expect("Failed to input as integer");
        numbers.insert(number % 42);
    }

    println!("{}", numbers.len());
}
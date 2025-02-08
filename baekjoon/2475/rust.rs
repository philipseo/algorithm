use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sum: u64 = input
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<u64>().unwrap().pow(2))
        .sum();

    println!("{}", sum % 10);
}

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values:Vec<u64> = input
        .as_mut_str()
        .split_whitespace()
        .take(3)
        .map(|split_value| split_value.parse().expect("Failed to parse input as integer"))
        .collect();

    println!("{}", values.iter().fold(0, |acc, &num| acc + num));
}

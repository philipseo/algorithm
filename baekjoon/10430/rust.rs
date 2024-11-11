use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values:Vec<i32> = input
        .as_mut_str()
        .split_whitespace()
        .take(3)
        .map(|split_value| split_value.parse().expect("Failed to parse input as integer"))
        .collect();

    let a = values[0];
    let b = values[1];
    let c = values[2];

    if a < 2 || a > 10000 || b < 2 || b > 10000 || c < 2 || c > 10000 {
        println!("a, b and c must be between 2 and 10000");
    } else {
        println!("{}", (a + b) % c);
        println!("{}", ((a % c) + (b % c)) % c);
        println!("{}", (a * b) % c);
        println!("{}", (a % c) * (b % c) % c);
    }
}
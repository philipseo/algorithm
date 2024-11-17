use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to input as integer");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i16> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|value| value.parse().expect("Failed to input as integer"))
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let v: i16 = input.trim().parse().expect("Failed to input as integer");

    println!("{}", numbers.iter().filter(|number| **number == v).count());
}
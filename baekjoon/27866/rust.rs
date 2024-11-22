use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s: String = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let i: usize = input.trim().parse().expect("Failed to input as integer");

    println!("{}", s.chars().nth(i - 1).unwrap());
}

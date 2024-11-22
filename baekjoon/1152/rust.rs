use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let result: Vec<&str> = input.trim().split_whitespace().collect();

    println!("{}", result.len());
}

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values:Vec<i32> = input
        .as_mut_str()
        .split_whitespace()
        .take(2)
        .map(|split_value| split_value.parse().expect("Failed to parse input as integer"))
        .collect();

    println!("{}", values[0] - values[1]);
}

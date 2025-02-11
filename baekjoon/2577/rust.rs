use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input: Vec<u32> = input
        .trim()
        .split_ascii_whitespace()
        .map(|value| value.parse().unwrap())
        .collect();
    let num = (input[0] * input[1] * input[2]).to_string();

    for i in '0'..='9' {
        println!("{}", num.chars().filter(|&c| c == i).count());
    }
}

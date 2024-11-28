use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let (mut n, b): (u32, u32) = {
        let (n_str, b_str) = input.trim().split_once(' ').unwrap();
        (n_str.parse().unwrap(), b_str.parse().unwrap())
    };

    let mut result = String::new();

    while n > 0 {
        let remainder = n % b;
        let digit = std::char::from_digit(remainder, b).unwrap();

        result.push(digit.to_ascii_uppercase());
        n /= b;
    }

    println!("{}", result.chars().rev().collect::<String>());
}

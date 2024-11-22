use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let (a, b): (u16, u16) = {
        let numbers: Vec<u16> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|number| number.chars().rev().collect::<String>().parse().unwrap())
            .collect();
        (numbers[0], numbers[1])
    };

    println!("{}", a.max(b));
}

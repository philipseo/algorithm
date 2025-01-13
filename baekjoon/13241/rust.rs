use std::io;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (a, b): (u64, u64) = {
        let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();
        (numbers[0], numbers[1])
    };

    println!("{}", (a * b) / gcd(a, b));
}

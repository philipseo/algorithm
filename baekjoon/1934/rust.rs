use std::io::{self, Read};

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
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let (a, b): (u64, u64) = {
            let numbers: Vec<u64> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();
            (numbers[0], numbers[1])
        };

        println!("{}", (a * b) / gcd(a, b));
    }
}

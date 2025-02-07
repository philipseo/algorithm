use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (a, b): (i64, i64) = {
        let numbers: Vec<i64> = input
            .trim()
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        (numbers[0], numbers[1])
    };

    println!("{}", (a + b) * (a - b));
}

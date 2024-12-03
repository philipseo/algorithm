use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (a, b, v): (u32, u32, u32) = {
        let values: Vec<u32> = input
            .trim()
            .split_whitespace()
            .take(3)
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1], values[2])
    };

    println!("{}", ((v - b) as f64 / (a - b) as f64).ceil() as u32);
}

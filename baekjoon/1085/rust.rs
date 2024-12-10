use std::io;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (x, y, w, h): (u16, u16, u16, u16) = {
        let values: Vec<u16> = input
            .trim()
            .split_whitespace()
            .take(4)
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1], values[2], values[3])
    };

    println!("{}", min(min(h - y, w - x), min(y, x)));
}

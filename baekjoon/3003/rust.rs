use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut black_pins: Vec<i8> = vec![1, 1, 2, 2, 2, 8];
    let white_pins: Vec<i8> = input
        .trim()
        .split_whitespace()
        .take(6)
        .map(|value| value.parse().unwrap())
        .collect();

    for i in 0..black_pins.len() {
        black_pins[i] = black_pins[i] - white_pins[i];
    }

    println!("{}", black_pins.iter().map(|&black_pin| black_pin.to_string()).collect::<Vec<String>>().join(" "));
}

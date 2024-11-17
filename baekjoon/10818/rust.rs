use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n:usize = input.trim().parse().expect("Failed to input as integer");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values:Vec<i32> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|value| value.parse().expect("Failed to input as integer"))
        .collect();

    let (min, max) = values.iter().skip(1).fold((values[0], values[0]), |(min, max), &value| {
        (min.min(value), max.max(value))
    });

    println!("{} {}", min, max);
}

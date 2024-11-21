use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to input as integer");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let scores: Vec<f32> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|score| score.parse().expect("Failed to input as integer"))
        .collect();

    let m = scores.iter().cloned().reduce(0.0, f32::max);
    let total_score: f32 = scores.iter().map(|&score| score / m * 100.0).sum();

    println!("{}", total_score / n as f32);
}
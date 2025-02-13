use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let test_cases: Vec<String> = input
        .split_ascii_whitespace()
        .skip(1)
        .map(|line| line.to_string())
        .collect();

    let mut results: Vec<String> = Vec::new();

    for test_case in test_cases {
        let mut score = 0;
        let mut sum = 0;

        for c in test_case.chars() {
            if c == 'O' {
                score += 1;
                sum += score;
            } else {
                score = 0;
            }
        }

        results.push(sum.to_string());
    }

    println!("{}", results.join("\n"));
}

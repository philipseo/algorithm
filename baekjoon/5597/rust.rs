use std::io;

fn main() {
    let mut input = String::new();

    for _ in 0..28 {
        io::stdin().read_line(&mut input).expect("Failed to read line");
    }

    let total_students: Vec<u8> = (1..=30).collect();
    let submitted: Vec<u8> = input
        .trim()
        .split_whitespace()
        .map(|value| value.parse().expect("Failed to input as integer"))
        .collect();
    let mut not_submitted: Vec<u8> = total_students
        .into_iter()
        .filter(|&student| !submitted.contains(&student))
        .collect();

    not_submitted.sort_unstable();

    println!("{}", not_submitted[0]);
    println!("{}", not_submitted[1]);
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (n, k): (usize, usize) = {
        let numbers: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        (numbers[0], numbers[1])
    };

    let mut queue: Vec<usize> = (1..=n).collect();
    let mut result: Vec<usize> = Vec::new();
    let mut index = 0;

    while !queue.is_empty() {
        index = (index + k - 1) % queue.len();
        result.push(queue.remove(index));
    }

    println!("<{}>", result.iter().map(|number| number.to_string()).collect::<Vec<_>>().join(", "));
}

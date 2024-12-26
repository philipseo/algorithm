use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let(n, k): (usize, usize) = {
        let values: Vec<usize> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1])
    };
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut scores: Vec<u32> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|score| score.parse().unwrap())
        .collect();

    scores.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", scores[k - 1]);
}

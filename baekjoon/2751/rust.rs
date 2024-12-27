use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut numbers: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().expect("Failed to parse number"))
        .collect();

    numbers.sort_unstable();

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for number in numbers {
        writeln!(out, "{}", number).unwrap();
    }
}

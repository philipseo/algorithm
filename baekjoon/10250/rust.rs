use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.trim().lines().skip(1);
    let mut results = Vec::new();

    for line in lines {
        let parts: Vec<u32> = line
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        let (h, _w, n) = (parts[0], parts[1], parts[2]);
        let floor = if n % h == 0 { h } else { n % h };
        let ho = (n as f64 / h as f64).ceil() as u32;

        results.push(format!("{}{:02}", floor, ho));
    }

    println!("{}", results.join("\n"));
}

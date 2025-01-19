use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input: Vec<&str> = input.trim().split_ascii_whitespace().collect();
    let test_cases: Vec<usize> = input[1..]
        .iter()
        .map(|&case| case.parse().unwrap())
        .collect();

    let max_number: usize = *test_cases.iter().max().unwrap();
    let mut is_primes: Vec<bool> = vec![true; max_number + 1];
    is_primes[0] = false;
    is_primes[1] = false;

    for i in 2..=(max_number as f64).sqrt() as usize {
        if is_primes[i] {
            for j in (i * i..=max_number).step_by(i) {
                is_primes[j] = false;
            }
        }
    }

    let mut results = Vec::new();
    for &n in &test_cases {
        let mut count = 0;
        for i in 2..=(n / 2) {
            if is_primes[i] && is_primes[n - i] {
                count += 1;
            }
        }

        results.push(count);
    }

    for result in results {
        println!("{}", result);
    }
}

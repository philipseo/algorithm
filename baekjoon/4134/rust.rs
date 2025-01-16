use std::io::{self, Read};

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _test_case: usize = lines.next().unwrap().parse().unwrap();
    let results: Vec<u64> = lines
        .map(|line| {
            let mut n: u64 = line.parse().unwrap();
            
            while !is_prime(n) {
                n += 1;
            }
            n
        })
        .collect();

    for result in results {
        println!("{}", result);
    }
}

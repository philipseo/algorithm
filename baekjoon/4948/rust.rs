use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let max: usize = 123456 * 2;
    let mut is_primes: Vec<bool> = vec![true; max + 1];
    is_primes[0] = false;
    is_primes[1] = false;

    for i in 2..=(max as f64).sqrt() as usize {
        if is_primes[i] {
            for j in (i * i..=max).step_by(i) {
                is_primes[j] = false;
            }
        }
    }
    
    let mut results = Vec::new();

    for line in input.lines() {
        let n: usize = line.parse().unwrap();
        
        if n == 0 {
            break;
        } else {
            let count = (n + 1..=2 * n)
                .filter(|&x| is_primes[x])
                .count();            

            results.push(count);
        }
    }

    for result in results {
        println!("{}", result);
    }
}

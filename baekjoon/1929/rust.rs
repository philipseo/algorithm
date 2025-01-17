use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut inputs = input.split_whitespace();
    let n: usize = inputs.next().unwrap().parse().unwrap();
    let m: usize = inputs.next().unwrap().parse().unwrap();

    let mut primes = vec![false; m + 1];
    for i in 2..=((m as f64).sqrt() as usize) {
        if !primes[i] {
            for j in (i * i..=m).step_by(i) {
                primes[j] = true;
            }
        }
    }

    let mut result = Vec::new();
    for i in n..=m {
        if i > 1 && !primes[i] {
            result.push(i);
        }
    }

    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\n"));
}

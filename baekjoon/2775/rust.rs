use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    let mut results = Vec::with_capacity(t);

    for _ in 0..t {
        let k: usize = lines.next().unwrap().parse().unwrap();
        let n: usize = lines.next().unwrap().parse().unwrap();

        let mut floor: Vec<u32> = (1..=n as u32).collect();

        for _ in 0..k {
            for j in 1..n {
                floor[j] += floor[j - 1];
            }
        }
        results.push(floor[n - 1]);
    }

    for result in results {
        println!("{}", result);
    }
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();
    let n: u32 = input.next().unwrap().parse().unwrap();
    let sizes: Vec<u32> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|size| size.parse().unwrap())
        .collect();
    let (t, p): (u32, u32) = {
        let values: Vec<u32> = input
            .next()
            .unwrap()
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1])
    };

    println!("{}", sizes.iter().map(|&size| size.div_ceil(t)).sum::<u32>());
    println!("{} {}", n / p, n % p);
}

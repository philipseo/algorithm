use std::io::{self, Read};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _n: usize = lines.next().unwrap().parse().unwrap();
    let positions: Vec<usize> = lines.map(|line| line.parse().unwrap()).collect();

    let mut gaps = Vec::new();
    for i in 1..positions.len() {
        gaps.push(positions[i] - positions[i - 1]);
    }

    let mut gcds = gaps[0];
    for &gap in &gaps[1..] {
        gcds = gcd(gcds, gap);
    }

    let total_trees: usize = gaps.iter().map(|&gap| (gap / gcds) - 1).sum();

    println!("{}", total_trees);
}

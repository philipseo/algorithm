use std::io::{stdin, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();
    let mut s: HashSet<&str> = HashSet::new();

    for _ in 0..n {
        s.insert(input.next().unwrap());
    }

    let mut result = 0;

    for _ in 0..m {
        if s.contains(input.next().unwrap()) {
            result += 1;
        }
    }

    println!("{}", result);
}

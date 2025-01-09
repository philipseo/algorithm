use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();
    let unheard: HashSet<String> = (0..n)
        .map(|_| input.next().unwrap().to_string())
        .collect();
    let unseen: Vec<String> = (0..m)
        .map(|_| input.next().unwrap().to_string())
        .collect();
    let mut result: Vec<String> = unseen
        .into_iter()
        .filter(|name| unheard.contains(name))
        .collect();

    result.sort();

    println!("{}", result.len());
    for name in result {
        println!("{}", name);
    }
}

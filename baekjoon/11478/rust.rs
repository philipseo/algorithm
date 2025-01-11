use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();

    let mut words = HashSet::new();

    for i in 0..input.len() {
        for j in i + 1..=input.len() {
            words.insert(&input[i..j]);
        }
    }

    println!("{}", words.len());
}

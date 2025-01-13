use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let n_cards: HashSet<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .take(n)
        .map(|card| card.parse::<i32>().unwrap())
        .collect();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let m_cards: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .take(m)
        .map(|card| card.parse::<i32>().unwrap())
        .collect();

    let result: Vec<u8> = m_cards
        .iter()
        .map(|&card| if n_cards.contains(&card) { 1 } else { 0 })
        .collect();

    println!("{}", result.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(" "));
}
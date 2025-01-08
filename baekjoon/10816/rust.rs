use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let n_cards: HashMap<i32, usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .take(n)
        .map(|card| card.parse::<i32>().unwrap())
        .fold(HashMap::with_capacity(n), |mut cards, card| {
            *cards.entry(card).or_insert(0) += 1;
            cards
        });
    let m: usize = lines.next().unwrap().parse().unwrap();
    let m_cards: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .take(m)
        .map(|card| card.parse::<i32>().unwrap())
        .collect();

    let result: Vec<usize> = m_cards
        .iter()
        .map(|&card| *n_cards.get(&card).unwrap_or(&0))
        .collect();

    println!("{}", result.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(" "));
}

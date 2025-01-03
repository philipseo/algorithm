use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    
    let _n: usize = input.next().unwrap().parse().unwrap();
    let coordinates: Vec<i32> = input.map(|s| s.parse().unwrap()).collect();

    let mut unique_coordinates = coordinates.clone();
    unique_coordinates.sort_unstable();
    unique_coordinates.dedup();

    let result: Vec<_> = coordinates
        .iter()
        .map(|&value| unique_coordinates.binary_search(&value).unwrap())
        .collect();

    let mut output = String::new();

    for (_, value) in result.iter().enumerate() {
        write!(output, "{} ", value).unwrap();
    }
    println!("{}", output.trim());
}

use std::io::{self, Read};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let _m: usize = input.next().unwrap().parse().unwrap();
    let mut poketmons_hash = HashMap::with_capacity(n);
    let mut poketmons_vec = Vec::with_capacity(n);

    for (index, poketmon) in input.by_ref().take(n).enumerate() {
        poketmons_hash.insert(poketmon, index + 1);
        poketmons_vec.push(poketmon);
    }

    let mut result = Vec::new();

    for question in input {
        if let Ok(index) = question.parse::<usize>() {
            result.push(poketmons_vec[index - 1].to_string());
        } else {
            result.push(poketmons_hash[question].to_string());
        }
    }

    println!("{}", result.join("\n"));
}

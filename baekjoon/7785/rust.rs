use std::io::{self, Read};
use std::collections::HashSet;
use std::cmp::Reverse;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut result: HashSet<String> = HashSet::with_capacity(n);

    for _ in 0..n {
        let name = input.next().unwrap().to_string();
        let action = input.next().unwrap();

        if action == "enter" {
            result.insert(name);
        } else if action == "leave" {
            result.remove(&name);
        }
    }

    let mut result = Vec::from_iter(result);
    result.sort_unstable_by_key(|name| Reverse(name.to_string()));

    print!("{}", result.join("\n"));
}

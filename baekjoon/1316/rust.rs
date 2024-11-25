use std::io;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let word = input.trim();
        let mut seen = HashSet::new();
        let mut previous_char: Option<char> = None;

        for char in word.chars() {
            if Some(char) != previous_char {
                if seen.contains(&char) {
                    n -= 1;
                    break;
                } else {
                    seen.insert(char);
                }
            }
            previous_char = Some(char);
        }
    }

    println!("{}", n);
}

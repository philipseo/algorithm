use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let l: usize = lines.next().unwrap().trim().parse().unwrap();
    let words = lines.next().unwrap();
    let r = 31;
    let m = 1234567891;
    let mut hash = 0;
    let mut power = 1;

    for i in 0..l {
        let char_value = (words.chars().nth(i).unwrap() as u64) - 96;
        hash = (hash + (char_value * power) % m) % m;
        power = (power * r) % m;
    }

    println!("{}", hash);

}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut word = input.trim().to_string();
    let croatia_alphabets = vec!["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];

    for croatia_alphabet in croatia_alphabets {
        word = word.replace(croatia_alphabet, "C");
    }

    println!("{}", word.len());
}

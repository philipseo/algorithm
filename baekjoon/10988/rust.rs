use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word = input.trim();
    let reverse_word = word.chars().rev().collect::<String>();

    println!("{}", if word == reverse_word { 1 } else { 0 });
}

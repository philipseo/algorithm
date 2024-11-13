use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut dices:Vec<i16> = input
        .as_mut_str()
        .split_whitespace()
        .take(3)
        .map(|dice| dice.parse().expect("Failed to parse dice as integer"))
        .collect();

    dices.sort();

    if dices[0] == dices[1] && dices[1] == dices[2] {
        println!("{}", 10000 + (dices[0] * 1000));
    } else if dices[0] == dices[1] || dices[1] == dices[2] {
        println!("{}", 1000 + (dices[1] * 100));
    } else {
        println!("{}", dices[2] * 100);
    }
}

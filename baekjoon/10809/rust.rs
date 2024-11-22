use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s = input.trim();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut result: Vec<i8> = Vec::new();
    
    for char in alphabet.chars() {
        let position = s.find(char).map_or(-1, |index| index as i8);
        result.push(position);
    }

    println!("{}", result.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}

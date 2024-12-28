use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: Vec<u8> = input
        .trim()
        .chars()
        .map(|number| number.to_digit(10).unwrap() as u8)
        .collect();

    n.sort_by(|a, b| b.cmp(a));

    let result: String = n.into_iter().map(|number| number.to_string()).collect();
    println!("{}", result);
}

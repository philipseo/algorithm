use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let result = (1..=n).fold(1, |acc, x| acc * x);

    println!("{}", result);
}

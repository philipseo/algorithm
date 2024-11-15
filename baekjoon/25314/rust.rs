use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: u16 = input.trim().parse().expect("Failed to input as integer");

    println!("{}int", "long ".repeat((n / 4) as usize));
}
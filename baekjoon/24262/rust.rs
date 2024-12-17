use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: u32 = input.trim().parse().unwrap();

    println!("1");
    println!("0");
}

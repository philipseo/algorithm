use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    println!("{}", (n * (n - 1) * (n - 2)) / 6);
    println!("3");
}

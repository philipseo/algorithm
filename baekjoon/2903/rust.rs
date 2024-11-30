use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();

    let side_length: u32 = 2_u32.pow(n as u32) + 1;

    println!("{}", side_length.pow(2));
}

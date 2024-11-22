use std::io;

fn main() {
    let mut input = String::with_capacity(1);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    println!("{}", input.trim().chars().next().unwrap() as u8);
}

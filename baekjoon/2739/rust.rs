use std::io;

fn main() {
    let mut input = String::with_capacity(1);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n:i8 = input.trim().parse().expect("Please enter a valid number");

    for i in 1..10 {
        println!("{} * {} = {}", n, i, n * i);
    }
}
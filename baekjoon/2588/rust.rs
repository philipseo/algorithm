use std::io;

fn main() {
    let mut input_a = String::with_capacity(3);
    io::stdin().read_line(&mut input_a).expect("Failed to input A");
    let a: i32 = input_a.trim().parse().expect("Please enter a valid number for A.");

    let mut input_b = String::with_capacity(3);
    io::stdin().read_line(&mut input_b).expect("Failed to input B");
    let b: i32 = input_b.trim().parse().expect("Please enter a valid number for B.");

    println!("{}", a * (b % 10));
    println!("{}", a * ((b / 10) % 10));
    println!("{}", a * (b / 100));
    println!("{}", a * b);
}

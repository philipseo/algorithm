use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n:i32 = input.trim().parse().expect("Failed to input as integer");
    let mut result: i32 = 0;

    for i in 1..=n {
        result += i;
    }

    println!("{}", result);
}
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let _n:usize = input.trim().parse().expect("Failed to input as integer");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers = input.trim();

    let mut sum: u32 = 0;

    for number in numbers.chars() {
        sum += number.to_string().parse::<u32>().unwrap();
    }

    println!("{}", sum);
}

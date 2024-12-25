use std::io;

fn main() {
    let mut input = String::new();
    let mut numbers = Vec::new();

    for _ in 1..=5 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let number: u16 = input.trim().parse().unwrap();
        numbers.push(number);
    }

    let sum: u16 = numbers.iter().sum();
    let average = sum / numbers.len() as u16;
    numbers.sort();
    let median = numbers[2];

    println!("{}\n{}", average, median);
}

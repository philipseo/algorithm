use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u16> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|number| number.parse().unwrap())
        .collect();

    let result = numbers.iter().filter(|&&number| {
        if number == 1 {
            return false;
        } else {
            for i in 2..=((number as f64).sqrt() as u16) {
                if number % i == 0 {
                    return false;
                }
            }

            return true;
        }
    }).count();

    println!("{result}");
}

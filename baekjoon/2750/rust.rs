use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut numbers = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        numbers.push(input.trim().parse::<i16>().unwrap());
    }

    numbers.sort();

    for number in numbers {
        println!("{}", number);
    }
}

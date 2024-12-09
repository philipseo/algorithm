use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap();
    let mut divisor: u32 = 2;
    let mut result: Vec<u32> = Vec::new();

    while n > 1 {
        if n % divisor == 0 {
            result.push(divisor);
            n /= divisor;
            divisor = 2;
        } else {
            divisor += 1;
        }
    }

    println!("{}", result.iter().map(|prime| prime.to_string()).collect::<Vec<String>>().join("\n"));
}

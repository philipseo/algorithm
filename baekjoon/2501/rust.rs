use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (n, k): (u16, u16) = {
        let values: Vec<u16> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1])
    };

    let mut count = 0;
    let mut result = 0;

    for i in 1..=n {
        if n % i == 0 {
            count += 1;
            if count == k {
                result = i;
                break;
            }
        }
    }

    println!("{}", result);
}

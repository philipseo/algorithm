use std::io;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn main() {
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).unwrap();
    let (a, b): (u32, u32) = {
        let values: Vec<u32> = input_a
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1])
    };
    let greatest_common_divisor = gcd(a, b);

    println!("{}", greatest_common_divisor);
    println!("{}", a * b / greatest_common_divisor);
}

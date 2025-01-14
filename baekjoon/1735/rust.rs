use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    
    gcd(b, a % b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (numerator1, demoninator1): (i32, i32) = {
        let numbers: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        (numbers[0], numbers[1])
    };

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let (numerator2, demoninator2): (i32, i32) = {
        let numbers: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        (numbers[0], numbers[1])
    };

    let numerator = numerator1 * demoninator2 + numerator2 * demoninator1;
    let demoninator = demoninator1 * demoninator2;
    let divisor = gcd(numerator, demoninator);
    
    println!("{} {}", numerator / divisor, demoninator / divisor);
}

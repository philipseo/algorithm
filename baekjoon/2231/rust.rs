use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let mut m:u32 = 0;
    
    for i in 1..n {
        let digit_sum: u32 = i
            .to_string()
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .sum();
        let sum = digit_sum + i;
        
        if sum == n {
            m = i;
            break;
        }
    }
    
    println!("{}", m);
}

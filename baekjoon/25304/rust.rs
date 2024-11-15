use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read total amount");
    let x:u64 = input.trim().parse().expect("Please enter a valid number");
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read totalItemCount");
    let n:usize = input.trim().parse().expect("Please enter a valid number");

    let mut result: u64 = 0;

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read item");
        let item: Vec<u64> = input
            .as_mut_str()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().expect("Please enter valid number"))
            .collect();

        result += item[0] * item[1];
    }

    println!("{}", if x == result { "Yes" } else { "No" });
}

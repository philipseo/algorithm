use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to input as integer");

    let mut results = Vec::new();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut chars = input.trim().chars();
        let first = chars.next().unwrap();
        let last = chars.last().unwrap_or(first);
        
        results.push(format!("{}{}", first, last));
    }

    for result in results {
        println!("{}", result);
    }
}

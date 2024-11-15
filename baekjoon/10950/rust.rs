use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t:usize = input.trim().parse().expect("Failed to T as integer");

    let mut results = Vec::with_capacity(t);

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let values:Vec<i8> = input
            .as_mut_str()
            .split_whitespace()
            .take(2)
            .map(|split_value| split_value.parse().expect("Failed to parse input"))
            .collect();
        
        results.push(values[0] + values[1]);
    }

    for result in results {
        println!("{}", result);
    }
}
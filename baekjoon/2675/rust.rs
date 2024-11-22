use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to input as integer");
    
    let mut result: Vec<String> = Vec::new();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let (r, s): (u8, String) = {
            let (r_str, s_str) = input.trim().split_once(' ').unwrap();
            (r_str.parse().unwrap(), s_str.to_string())
        };

        result.push(s.chars().map(|char| char.to_string().repeat(r as usize)).collect());
    }

    println!("{}", result.join("\n"));
}

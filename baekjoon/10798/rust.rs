use std::io;

fn main() {
    let mut input = String::new();
    let mut lines = Vec::new();
    let mut max_length = 0;

    for _ in 0..5 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let value = input.trim().to_string();
        max_length = max_length.max(value.len());
        lines.push(value);
    }

    let mut result = String::new();

    for i in 0..max_length {
        for line in &lines {
            if let Some(char) = line.chars().nth(i) {
                result.push(char);
            }
        }
    }

    println!("{}", result);
}

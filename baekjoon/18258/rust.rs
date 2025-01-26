use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _n = lines.next();
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut result: Vec<String> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let command = parts[0];

        match command {
            "push" => {
                let value: i32 = parts[1].parse().unwrap();
                queue.push_back(value);
            },
            "pop" => result.push(queue.pop_front().unwrap_or(-1).to_string()),
            "size" => result.push(queue.len().to_string()),
            "empty" => result.push(if queue.is_empty() { "1" } else { "0" }.to_string()),
            "front" => result.push(queue.front().unwrap_or(&-1).to_string()),
            "back" => result.push(queue.back().unwrap_or(&-1).to_string()),
            _ => (),
        }
    }

    println!("{}", result.join("\n"));
}
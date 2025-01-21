use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _n: usize = lines.next().unwrap().parse().unwrap();
    let mut stack: Vec<i32> = Vec::new();
    let mut result: Vec<i32> = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        let command: usize = parts.next().unwrap().parse::<usize>().unwrap();

        match command {
            1 => {
                let value: i32 = parts.next().unwrap().parse().unwrap();
                stack.push(value);
            },
            2 => {
                if stack.is_empty() {
                    result.push(-1);
                } else {
                    result.push(stack.pop().unwrap());
                }
            }
            3 => {
                result.push(stack.len() as i32);
            }
            4 => {
                result.push(if stack.is_empty() { 1 } else { 0 });
            }
            5 => {
                if stack.is_empty() {
                    result.push(-1);
                } else {
                    result.push(*stack.last().unwrap());
                }
            }
            _ => {},
        }
    }

    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n"));
}

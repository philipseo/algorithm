use std::io::{self, Read};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _n: usize = lines.next().unwrap().parse().unwrap();
    let mut deque: VecDeque<i32> = VecDeque::new();
    let mut result: Vec<i32> = Vec::new();

    for line in lines {
        let mut parts = line.split_whitespace();
        let command: usize = parts.next().unwrap().parse::<usize>().unwrap();

        match command {
            1 => {
                let value: i32 = parts.next().unwrap().parse().unwrap();
                deque.push_front(value);
            },
            2 => {
                let value: i32 = parts.next().unwrap().parse().unwrap();
                deque.push_back(value);
            }
            3 => {
                result.push(deque.pop_front().unwrap_or(-1));
            }
            4 => {
                result.push(deque.pop_back().unwrap_or(-1));
            }
            5 => {
                result.push(deque.len() as i32);
            }
            6 => {
                result.push(if deque.is_empty() { 1 } else { 0 });
            }
            7 => {
                result.push(*deque.front().unwrap_or(&-1));
            }
            8 => {
                result.push(*deque.back().unwrap_or(&-1));
            }
            _ => {},
        }
    }

    println!("{}", result.iter().map(|value| value.to_string()).collect::<Vec<_>>().join("\n"));
}

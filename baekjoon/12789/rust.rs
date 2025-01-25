use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _n = lines.next().unwrap();
    let mut numbers: VecDeque<u16> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect();

    let mut stack: Vec<u16> = Vec::new();
    let mut current_number = 1;

    while !numbers.is_empty() || !stack.is_empty() {
        if let Some(&front) = numbers.front() {
            if front == current_number {
                numbers.pop_front();
                current_number += 1;
                continue;
            }
        }

        if let Some(&top) = stack.last() {
            if top == current_number {
                stack.pop();
                current_number += 1;
                continue;
            }
        }

        if let Some(front) = numbers.pop_front() {
            stack.push(front);
        } else {
            break;
        }
    }

    println!("{}", if stack.is_empty() { "Nice" } else { "Sad" });
}

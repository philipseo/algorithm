use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines: Vec<&str> = input.lines().collect();
    lines.pop();

    let mut result = Vec::new();

    for line in lines {
        let mut stack = Vec::new();
        let mut is_balanced = true;

        for char in line.chars() {
            match char {
                '(' | '[' => stack.push(char),
                ')' => {
                    if let Some('(') = stack.pop() {
                        continue;
                    } else {
                        is_balanced = false;
                        break;
                    }
                }
                ']' => {
                    if let Some('[') = stack.pop() {
                        continue;
                    } else {
                        is_balanced = false;
                        break;
                    }
                }
                _ => {},
            }
        }

        if is_balanced && stack.is_empty() {
            result.push("yes");
        } else {
            result.push("no");
        }
    }

    println!("{}", result.join("\n"));
}

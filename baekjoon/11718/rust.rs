use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = Vec::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(text) => lines.push(text),
            Err(_) => break,
        }
    }

    for line in lines {
        println!("{}", line);
    }
}

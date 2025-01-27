use std::io;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut queue: VecDeque<usize> = (1..=n).collect();

    while queue.len() > 1 {
        queue.pop_front();
        let card = queue.pop_front().unwrap();
        queue.push_back(card);
    }

    println!("{}", queue.pop_front().unwrap());
}

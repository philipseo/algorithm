use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _n = lines.next().unwrap().parse::<usize>().unwrap();
    let mut stack = Vec::new();

    for line in lines {
        let num = line.parse::<i32>().unwrap();
        
        if num == 0 {
            stack.pop();
        } else {
            stack.push(num);
        }
    }

    let sum: i32 = stack.iter().sum();

    println!("{}", sum);
}

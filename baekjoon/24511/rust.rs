use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let a: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let b: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let c: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let mut result = Vec::new();

    for i in (0..n).rev() {
        if result.len() == m {
            break;
        }
        if a[i] == "0" {
            result.push(b[i].to_string());
        }
    }

    for i in 0..m {
        if result.len() == m {
            break;
        }
        result.push(c[i].to_string());
    }

    println!("{}", result.join(" "));
}

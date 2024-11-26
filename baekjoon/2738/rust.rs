use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n, m): (usize, usize) = {
        let values: Vec<usize> = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1])
    };

    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 0..n * 2 {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<u16> = line.split_whitespace().map(|value| value.parse().unwrap()).collect();
        if i < n {
            a.push(row);
        } else {
            b.push(row);
        }
    }

    let mut result = String::new();

    for i in 0..n {
        for j in 0..m {
            result.push_str(&format!(
                "{}{}",
                a[i][j] + b[i][j],
                if j != m - 1 { " " } else { "" }
            ));
        }
        if i != n - 1 {
            result.push('\n');
        }
    }

    println!("{}", result);
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (n, m): (usize, u32) = {
        let values: Vec<usize> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1] as u32)
    };

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let cards: Vec<u32> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|card| card.parse().unwrap())
        .collect();

    let mut max_sum = 0;

    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let sum = cards[i] + cards[j] + cards[k];
                
                if sum <= m && sum > max_sum {
                    max_sum = sum;
                }
            }
        }
    }

    println!("{}", max_sum);
}

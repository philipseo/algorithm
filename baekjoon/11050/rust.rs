use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (n, k): (usize, usize) = {
        let (n_str, k_str) = input.trim().split_once(' ').unwrap();
        (n_str.parse().unwrap(), k_str.parse().unwrap())
    };

    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=n {
        for j in (1..=std::cmp::min(i, k)).rev() {
            dp[j] += dp[j - 1];
        }
    }

    println!("{}", dp[k]);
}

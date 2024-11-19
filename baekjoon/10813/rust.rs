use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let (n, m): (usize, usize) = {
        let values: Vec<usize> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().expect("Failed to input as integer"))
            .collect();
        (values[0], values[1])
    };
    let mut baskets: Vec<usize> = (1..=n).collect();

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let (i, j): (usize, usize) = {
            let values: Vec<usize> = input
                .trim()
                .split_whitespace()
                .take(2)
                .map(|value| value.parse().expect("Failed to input as integer"))
                .collect();
            (values[0] - 1, values[1] - 1)
        };

        baskets.swap(i, j);
    }

    println!("{}", baskets.iter().map(|&ball| ball.to_string()).collect::<Vec<String>>().join(" "));
}

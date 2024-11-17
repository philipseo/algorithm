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

    let mut baskets:Vec<u8> = vec![0; n];
    
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let (i, j, k): (usize, usize, u8) = {
            let values: Vec<u8> = input
                .trim()
                .split_whitespace()
                .map(|value| value.parse().expect("Failed to input as integer"))
                .collect();
            (values[0] as usize - 1, values[1] as usize - 1, values[2])
        };

        for index in i ..=j {
            baskets[index] = k;
        } 
    }

    println!("{}", baskets.iter().map(|&ball| ball.to_string()).collect::<Vec<String>>().join(" "));
}

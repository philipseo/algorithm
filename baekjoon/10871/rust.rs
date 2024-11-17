use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let (n, x): (usize, u16) = {
        let values: Vec<u16> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().expect("Failed to input as integer"))
            .collect();
        (values[0] as usize, values[1])
    };

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a:Vec<u16> = input
        .trim()
        .split_whitespace()
        .take(n)
        .map(|value| value.parse().expect("Failed to input as integer"))
        .collect();

    let result = a.into_iter().filter(|value| *value < x).map(|value| value.to_string()).collect::<Vec<_>>().join(" ");

    println!("{}", result);
}

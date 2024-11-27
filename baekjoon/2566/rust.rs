use std::io;

fn main() {
    let mut input = String::new();
    let mut max: u8 = u8::MIN;
    let mut position = (0, 0);

    for i in 0..9 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<u8> = input
            .trim()
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();

        for (j, &value) in row.iter().enumerate() {
            if value >= max {
                max = value;
                position = (i + 1, j + 1);
            }
        }
    }

    println!("{}", max);
    println!("{} {}", position.0, position.1);
}

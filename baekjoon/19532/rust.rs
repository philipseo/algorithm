use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i16> = input
        .trim()
        .split_whitespace()
        .take(6)
        .map(|number| number.parse().unwrap())
        .collect();
    let (a, b, c, d, e, f) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5]);
    
    'outer: for x in -999..=999 {
        for y in -999..=999 {
            if (a * x) + (b * y) == c && (d * x) + (e * y) == f {
                println!("{} {}", x, y);
                break 'outer;
            }
        }
    }
}

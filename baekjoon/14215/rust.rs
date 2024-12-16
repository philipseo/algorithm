use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut sides: Vec<u16> = input
        .trim()
        .split_whitespace()
        .take(3)
        .map(|side| side.parse().unwrap())
        .collect();
    sides.sort();

    let (a, b, mut c) = (sides[0], sides[1], sides[2]);

    while (a + b) <= c {
        c -= 1;
    }

    println!("{}", a + b + c);
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut result = Vec::new();

    for line in input.lines() {
        let mut sides: Vec<u64> = line
            .split_whitespace()
            .map(|side| side.parse().unwrap())
            .collect();

        if sides == vec![0, 0, 0] {
            break;
        } else {
            sides.sort_unstable();

            if sides[0].pow(2) + sides[1].pow(2) == sides[2].pow(2) {
                result.push("right");
            } else {
                result.push("wrong");
            }
        }
    }

    println!("{}", result.join("\n"));
}

use std::io;

fn main() {
    let mut input = String::new();
    let mut result = Vec::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let mut sides: Vec<u16> = input
            .trim()
            .split_whitespace()
            .take(3)
            .map(|side| side.parse().unwrap())
            .collect();

        if sides == vec![0, 0, 0] {
            break;
        } else {
            sides.sort();
            let (a, b, c) = (sides[0], sides[1], sides[2]);

            if a + b <= c {
                result.push("Invalid");
            } else if a == b && b == c {
                result.push("Equilateral");
            } else if a == b || b == c || a == c {
                result.push("Isosceles");
            } else {
                result.push("Scalene");
            }
            
            input.clear();
        }
    }

    println!("{}", result.join("\n"));
}

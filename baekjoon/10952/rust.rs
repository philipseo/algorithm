use std::io::{self, Write};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout());

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let values:Vec<u8> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.trim().parse().expect("Failed to input as integer"))
            .collect();

        let a:u8 = values[0];
        let b:u8 = values[1];

        if a == 0 && b == 0 {
            break;
        } else {
            writeln!(stdout, "{}", a + b).unwrap();
        }
    }

    stdout.flush().unwrap();
}

use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut stdout = io::BufWriter::new(io::stdout());
    let n: usize = input.trim().parse().expect("Failed to input as integer");

    for i in 1..=n {
        writeln!(stdout, "{}", "*".repeat(i)).unwrap();
    }

    stdout.flush().unwrap();
}
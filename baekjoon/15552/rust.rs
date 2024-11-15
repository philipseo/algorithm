use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let values:Vec<u16> = line
                .split_whitespace()
                .take(2)
                .map(|value| value.parse().expect("Failed to input as integer"))
                .collect();
            writeln!(stdout, "{}", values[0] + values[1]).unwrap();
        }
    }

    stdout.flush().unwrap();
}

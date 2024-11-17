use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());
    let lines = stdin.lock().lines();
    
    for line in lines {
        if let Ok(line) = line {
            let trim_line = line.trim();

            if trim_line.is_empty() {
                break;
            } else {
                let values: Vec<u8> = trim_line
                    .split_whitespace()
                    .take(2)
                    .map(|value| value.parse().expect("Failed to input as integer"))
                    .collect();

                writeln!(stdout, "{}", values[0] + values[1]).unwrap();
            }
        }
    }
    
    stdout.flush().unwrap();
}
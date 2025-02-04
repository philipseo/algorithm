use std::io::{self, BufWriter, Write};

fn main() {
    let mut input = String::with_capacity(1);
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for i in 1..=n {
        writeln!(writer, "{}", i).unwrap();
    }
}
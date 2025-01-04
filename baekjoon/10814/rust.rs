use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut members: Vec<(u8, String)> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut line = lines.next().unwrap().split_whitespace();
        let age: u8 = line.next().unwrap().parse().unwrap();
        let name: String = line.next().unwrap().to_string();
        members.push((age, name));
    }

    members.sort_by(|a, b| a.0.cmp(&b.0));

    let mut output = BufWriter::new(io::stdout());

    for (age, name) in members {
        writeln!(output, "{} {}", age, name).unwrap();
    }
}

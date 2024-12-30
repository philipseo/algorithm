use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut coordinates: Vec<(i32, i32)> = Vec::with_capacity(n);

    for line in lines.take(n) {
        let coords: Vec<i32> = line
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        coordinates.push((coords[0], coords[1]));
    }

    coordinates.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    for (x, y) in coordinates {
        writeln!(handle, "{} {}", x, y).unwrap();
    }
}

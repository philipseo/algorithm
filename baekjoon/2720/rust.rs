use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut stdout = io::BufWriter::new(io::stdout());
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut change: u16 = input.trim().parse().unwrap();
        
        let quarter = change / 25;
        change %= 25;

        let dime = change / 10;
        change %= 10;

        let nickel = change / 5;
        let penny = change % 5;

        writeln!(stdout, "{quarter} {dime} {nickel} {penny}").unwrap();
    }

    stdout.flush().unwrap();
}

use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut count_array = [0; 10001];

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let number: usize = input.trim().parse().unwrap();
        count_array[number] += 1;
    }

    let mut output = io::BufWriter::with_capacity(4096000, io::stdout());

    for i in 1..=10000 {
        if count_array[i] > 0 {
            for _ in 0..count_array[i] {
                write!(&mut output, "{}\n", i).unwrap();
            }
        }
    }
    
    Ok(())
}

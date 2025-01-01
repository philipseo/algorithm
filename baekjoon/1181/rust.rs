use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut words: Vec<String> = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        words.push(input.trim().to_string());
    }
    
    words.sort_by(|a, b| {
        if a.len() != b.len() {
            a.len().cmp(&b.len())
        } else {
            a.cmp(b)
        }
    });
    words.dedup();

    for word in words {
        println!("{}", word);
    }
}

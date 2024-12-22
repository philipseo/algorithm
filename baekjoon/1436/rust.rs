use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut count = 0;
    let mut num = 665;

    while count != n {
        num += 1;
        if num.to_string().contains("666") {
            count += 1;
        }
    }

    println!("{}", num);
}

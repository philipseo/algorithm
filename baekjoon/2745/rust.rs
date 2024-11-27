use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (n, b): (String, u8) = {
        let (n_str, b_str) = input.trim().split_once(' ').unwrap();
        (n_str.to_string(), b_str.parse().unwrap())
    };

    println!("{}", u32::from_str_radix(&n, b.into()).unwrap());
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split_ascii_whitespace();
    let a = lines.next().unwrap();
    let b = lines.next().unwrap();
    let c = lines.next().unwrap();

    println!("{}", 
        a.parse::<i32>().unwrap()
        + b.parse::<i32>().unwrap()
        - c.parse::<i32>().unwrap()
    );

    println!("{}", 
        format!("{}{}", a, b)
            .parse::<i32>()
            .unwrap()
        - c.parse::<i32>().unwrap()
    )
}

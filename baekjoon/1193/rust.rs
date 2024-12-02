use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut x: u32 = input.trim().parse().unwrap();
    let mut index: u32 = 1;

    while x > index {
        x -= index;
        index += 1;
    }

    if index % 2 == 0 {
        println!("{}/{}", x, index - x + 1);
    } else {
        println!("{}/{}", index - x + 1, x);
    }
}

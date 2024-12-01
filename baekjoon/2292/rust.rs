use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let mut layer = 1;
    let mut range = 1;

    while range < n {
        range += 6 * layer;
        layer += 1;
    }

    println!("{}", layer);
}

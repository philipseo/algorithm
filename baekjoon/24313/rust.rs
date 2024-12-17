use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (a1, a0): (i16, i16) = {
        let values: Vec<i16> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|value| value.parse().unwrap())
            .collect();
        (values[0], values[1])
    };

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let c: i16 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n0: i16 = input.trim().parse().unwrap();

    let f_n = (a1 * n0) + a0;
    let g_n = c * n0;

    if a1 <= c && f_n <= g_n {
        println!("1");
    } else {
        println!("0");
    }
}

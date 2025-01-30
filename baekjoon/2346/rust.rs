use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut balloons: Vec<(usize, i32)> = input
        .trim()
        .split_whitespace()
        .enumerate()
        .map(|(i, number)| (i + 1, number.parse::<i32>().unwrap()))
        .collect();

    let mut result = Vec::with_capacity(n);
    let mut index = 0;

    while !balloons.is_empty() {
        let (position, move_by) = balloons.remove(index);
        result.push(position);

        if balloons.is_empty() {
            break;
        } else {
            let len = balloons.len();

            if move_by > 0 {
                index = (index + (move_by - 1) as usize) % len;
            } else {
                index = (index as isize + move_by as isize).rem_euclid(len as isize) as usize;
            }
        }
    }

    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

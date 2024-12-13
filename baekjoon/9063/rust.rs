use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n = input.trim().parse().unwrap();
    let mut x_coordinates = Vec::new();
    let mut y_coordinates = Vec::new();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let (x, y): (i32, i32) = {
            let values: Vec<i32> = input
                .trim()
                .split_whitespace()
                .take(2)
                .map(|value| value.parse().unwrap())
                .collect();
            (values[0], values[1])
        };

        x_coordinates.push(x);
        y_coordinates.push(y);
    }

    let width = *x_coordinates.iter().max().unwrap() - *x_coordinates.iter().min().unwrap();
    let height = *y_coordinates.iter().max().unwrap() - *y_coordinates.iter().min().unwrap();

    println!("{}", width * height);
}

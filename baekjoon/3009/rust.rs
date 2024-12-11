use std::io;

fn main() {
    let mut input = String::new();
    let mut x_coordinates = Vec::new();
    let mut y_coordinates = Vec::new();

    for _ in 0..=2 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let (x, y): (u16, u16) = {
            let values: Vec<u16> = input
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

    x_coordinates.sort();
    y_coordinates.sort();

    let x = if x_coordinates[0] == x_coordinates[1] { x_coordinates[2] } else { x_coordinates[0] };
    let y = if y_coordinates[0] == y_coordinates[1] { y_coordinates[2] } else { y_coordinates[0] };
    
    println!("{} {}", x, y);
}

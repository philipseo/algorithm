use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut paper = vec![vec![false; 100]; 100];

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let (x, y): (usize, usize) = {
            let values: Vec<usize> = input
                .trim()
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();
            (values[0], values[1])
        };

        for dark_x in 0..10 {
            for dark_y in 0..10 {
                paper[x + dark_x][y + dark_y] = true;
            }
        }
    }

    let mut size = 0;
    for row in paper.iter() {
        size += row.iter().filter(|&&cell| cell).count();
    }

    println!("{}", size);
}

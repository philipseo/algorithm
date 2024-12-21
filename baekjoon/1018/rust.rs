use std::io;
use std::cmp;

fn get_count(x: usize, y: usize, board: &Vec<Vec<char>>, pattern: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    
    for i in 0..8 {
        for j in 0..8 {
            if board[x + i][y + j] != pattern[i][j] {
                count += 1;
            }
        }
    }
    
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (n, m): (usize, usize) = {
        let values: Vec<usize> = input
            .trim()
            .split_whitespace()
            .take(2)
            .map(|number| number.parse().unwrap())
            .collect();
        (values[0], values[1])
    };
    
    let mut board = Vec::new();
    
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line: Vec<char> = input.trim().chars().collect();
        board.push(line);
    }
    
    let white_board = vec![
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
    ];

    let black_board = vec![
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
        vec!['B', 'W', 'B', 'W', 'B', 'W', 'B', 'W'],
        vec!['W', 'B', 'W', 'B', 'W', 'B', 'W', 'B'],
    ];
    
    let mut min_count = usize::MAX;
    
    for i in 0..=n - 8 {
        for j in 0..=m - 8 {
            let white_count = get_count(i, j, &board, &white_board);
            let black_count = get_count(i, j, &board, &black_board);
            min_count = cmp::min(min_count, cmp::min(white_count, black_count));
        }
    }
    
    println!("{}", min_count);
}

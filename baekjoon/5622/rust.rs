use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let alphabet_length: usize = input.trim().len();
    let mut result: u16 = alphabet_length as u16;

    for i in 0..alphabet_length {
        let char: char = input.chars().nth(i).unwrap();
        match char {
            'A' | 'B' | 'C' => result += 2,
            'D' | 'E' | 'F' => result += 3,
            'G' | 'H' | 'I' => result += 4,
            'J' | 'K' | 'L' => result += 5,
            'M' | 'N' | 'O' => result += 6,
            'P' | 'Q' | 'R' | 'S' => result += 7,
            'T' | 'U' | 'V' => result += 8,
            'W' | 'X' | 'Y' | 'Z' => result += 9,
            _ => result += 1
        }
    }

    println!("{}", result);
}

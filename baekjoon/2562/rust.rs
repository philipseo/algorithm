use std::io;

fn main() {
    let mut input = String::new();
    let mut max = 0;
    let mut position = 1;
    
    for i in 0..9 {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let value: u8 = input.trim().parse().expect("Failed to input as integer");
        
        if value > max {
            max = value;
            position = i + 1;
        }
        
        input.clear();
    }

    println!("{}", max);
    println!("{}", position);
}
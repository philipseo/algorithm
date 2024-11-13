use std::io;

fn main() {
    let mut input = String::with_capacity(5);

    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let values:Vec<i8> = input
        .as_mut_str()
        .split_whitespace()
        .take(2)
        .map(|split_value| split_value.parse().expect("Failed to parse input as integer"))
        .collect();
    let mut hour = values[0];
    let mut minute = values[1] - 45;

    if minute < 0 {
        minute += 60;
        hour -= 1;

        if hour < 0 {
            hour = 23;
        }
    }

    println!("{} {}", hour, minute);
}
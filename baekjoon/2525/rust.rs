use std::io;

fn main() {
    let mut input_current_time = String::new();
    io::stdin().read_line(&mut input_current_time).expect("Failed to input current time");

    let mut input_cook_time = String::new();
    io::stdin().read_line(&mut input_cook_time).expect("Failed to input cook time");
    let cook_time:i16 = input_cook_time.trim().parse().expect("Please enter a valid number");

    let current_time:Vec<i16> = input_current_time
        .as_mut_str()
        .split_whitespace()
        .take(2)
        .map(|current_time| current_time.parse().expect("Failed to parse current time as integer"))
        .collect();

    let mut current_hour = current_time[0];
    let mut current_minute = current_time[1];

    current_minute += cook_time;

    if current_minute >= 60 {
        current_hour += current_minute / 60;
        current_minute %= 60;

        if current_hour > 23 {
            current_hour -= 24
        }
    }

    println!("{} {}", current_hour, current_minute);
}

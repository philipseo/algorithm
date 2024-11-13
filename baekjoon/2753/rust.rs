use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let year: i16 = input.trim().parse().expect("Failed to parse input as integer");

    if year < 1 || year > 4000 {
        println!("Please enter a year between 1 and 4000.");
    } else {
        let is_leap_year = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;

        println!("{}", if is_leap_year { 1 } else { 0 });
    }
}

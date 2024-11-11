use std::io;

fn main() {
    let mut input = String::with_capacity(4);

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let year:i16 = input
        .trim()
        .parse()
        .expect("Failed to parse input");

    if year < 1000 || year > 3000 {
        println!("The year must be between 1000 and 3000.");
    } else {
        println!("{}", year - 543);
    }
}
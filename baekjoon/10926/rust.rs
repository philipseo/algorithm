use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Faild to read line");

    let trim_input = input.trim();

    if trim_input.chars().all(|c| c.is_lowercase() || c.is_digit(10)) && trim_input.len() <= 50 {
        println!("{}??!", trim_input);
    } else {
        println!("The ID can only contain lowercase letters, numbers, and up to 50 characters.")
    }
}

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let result = input
        .chars()
        .map(|char| if char.is_ascii_lowercase() { char.to_ascii_uppercase() } else { char.to_ascii_lowercase() })
        .collect::<String>();
    
    println!("{}", result);
}

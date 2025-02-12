use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1 2 3 4 5 6 7 8" => println!("ascending"),
        "8 7 6 5 4 3 2 1" => println!("descending"),
        _ => println!("mixed")
        
    }
}

use std::io;

fn main() {
    let mut input_x = String::new();
    io::stdin().read_line(&mut input_x).expect("Failed to input X");
    let x:i16 = input_x.trim().parse().expect("Please enter a valid number");

    let mut input_y = String::new();
    io::stdin().read_line(&mut input_y).expect("Failed to input Y");
    let y:i16 = input_y.trim().parse().expect("Please enter a valid number");

    if x == 0 || y == 0 {
        eprintln!("Please enter a coordinate that is a non-zero positive or negative number.");
    } else {
        let quadrant = match (x > 0, y > 0) {
            (true, true) => "1",
            (false, true) => "2",
            (false, false) => "3",
            (true, false) => "4"
        };
        
        println!("{}", quadrant);
    }
}

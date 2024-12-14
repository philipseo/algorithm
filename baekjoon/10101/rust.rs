use std::io;

fn main() {
    let mut input = String::new();
    let mut angles = Vec::new();

    for _ in 0..3 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let angle: u32 = input.trim().parse().unwrap();
        angles.push(angle);
    }

    let sum: u32 = angles.iter().sum();

    if sum != 180 {
        println!("Error");
    } else if angles[0] == 60 && angles[1] == 60 && angles[2] == 60 {
        println!("Equilateral");
    } else if angles[0] == angles[1] || angles[0] == angles[2] || angles[1] == angles[2] {
        println!("Isosceles");
    } else {
        println!("Scalene");
    }
}
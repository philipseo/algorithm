use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    
    let grade_to_point: HashMap<&str, f64> = [
        ("A+", 4.5),
        ("A0", 4.0),
        ("B+", 3.5),
        ("B0", 3.0),
        ("C+", 2.5),
        ("C0", 2.0),
        ("D+", 1.5),
        ("D0", 1.0),
        ("F", 0.0),
    ].iter().cloned().collect();

    let mut total_point = 0.0;
    let mut total_avg_major_point = 0.0;
    
    for line in lines {
        let line = line.unwrap();
        let values: Vec<&str> = line.trim().split_whitespace().collect();
        let point: f64 = values[1].parse().unwrap();
        let grade = values[2];

        if grade == "P" {
            continue;
        } else {
            total_point += point;
            total_avg_major_point += point * grade_to_point.get(grade).unwrap();
        }
    }

    println!("{:.6}", total_avg_major_point / total_point);
}

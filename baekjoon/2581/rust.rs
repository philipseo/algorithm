use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let m = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    let mut result = Vec::new();

    for i in m..=n {
        if i >= 2 {
            let mut is_prime = true;
            
            for j in 2..=((i as f64).sqrt() as u32) {
                if i % j == 0 {
                    is_prime = false;
                    break;
                }
            }

            if is_prime {
                result.push(i);
            }
        }
    }

    if result.is_empty() {
        println!("-1");
    } else {
        let sum: u32 = result.iter().sum();
        let min = result[0];
        println!("{sum}\n{min}");
    }
}

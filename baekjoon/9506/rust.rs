use std::io;
use std::fmt::Write;

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        
        if n == -1 {
            break;
        } else {
            let mut sum = 0;
            let mut array = Vec::new();

            for i in 1..n {
                if n % i == 0 {
                    sum += i;
                    array.push(i);
                }
            }

            if n == sum {
                let result = array.iter().map(ToString::to_string).collect::<Vec<String>>().join(" + ");
                writeln!(output, "{} = {}", n, result).unwrap();
            } else {
                writeln!(output, "{} is NOT perfect.", n).unwrap();
            }
        }
    }

    print!("{}", output);
}

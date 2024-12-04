use std::io;

fn main() {
    let mut input = String::new();
    let mut result = Vec::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let (first, second): (u16, u16) = {
            let numbers: Vec<u16> = input
                .trim()
                .split_whitespace()
                .take(2)
                .map(|number| number.parse().unwrap())
                .collect();
            (numbers[0], numbers[1])
        };

        if first == 0 && second == 0 {
            break;
        } else {
            input.clear();
    
            if second % first == 0 {
                result.push("factor");
            } else if first % second == 0 {
                result.push("multiple");
            } else {
                result.push("neither");
            }
        }
    }
    
    println!("{}", result.join("\n"));
}

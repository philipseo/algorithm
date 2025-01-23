use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _t: usize = lines.next().unwrap().parse().unwrap();
    let ps_list: Vec<&str> = lines.collect();

    let mut result = Vec::new();
    
    for ps in ps_list {
        let mut stack: isize = 0;
        let mut is_vps = true;

        for p in ps.chars() {
            match p {
                '(' => stack += 1,
                ')' => stack -= 1,
                _ => continue,
            }

            if stack < 0 {
                is_vps = false;
                break;
            }
        }

        if stack != 0 {
            is_vps = false;
        }

        result.push(if is_vps { "YES" } else { "NO" });
    }

    println!("{}", result.join("\n"));
}

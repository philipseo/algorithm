use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let loop_count = (2 * n) - 1;

    for i in 1..=loop_count {
        let (blank, star): (String, String);

        if i <= n {
            blank = " ".repeat(n - i);
            star = "*".repeat(i + i - 1);
        } else {
            let reverse_loop_count = loop_count - i + 1;
            blank = " ".repeat(i - n);
            star = "*".repeat(reverse_loop_count + reverse_loop_count - 1);
        }

        println!("{}{}", blank, star);
    }
}

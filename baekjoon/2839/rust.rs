use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: i32 = input.trim().parse().unwrap();
    let mut count = 0;

    while n > 0 {
        if n % 5 == 0 {
            count += n / 5;
            break;
        } else {
            n -= 3;
            count += 1;
        }
    }

    if n < 0 {
        count = -1;
    }

    println!("{}", count);
}

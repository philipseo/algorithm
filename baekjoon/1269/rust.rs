use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let mut input = input.trim().lines();

	let (a_length, b_length): (usize, usize) = {
		let values: Vec<usize> = input
			.next()
			.unwrap()
			.split_whitespace()
			.map(|value| value.parse().unwrap())
			.collect();
		(values[0], values[1])
	};
	let a_set: HashSet<u32> = input
		.next()
		.unwrap()
		.split_whitespace()
		.take(a_length)
		.map(|number| number.parse().unwrap())
		.collect();
	let b_set: HashSet<u32> = input
		.next()
		.unwrap()
		.split_whitespace()
		.take(b_length)
		.map(|number| number.parse().unwrap())
		.collect();

	let mut count = 0;

	for &a in &a_set {
        if !b_set.contains(&a) {
            count += 1;
        }
    }

    for &b in &b_set {
        if !a_set.contains(&b) {
            count += 1;
        }
    }

    println!("{}", count);
}
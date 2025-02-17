use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let mut lines: Vec<&str> = input
		.trim()
		.lines()
		.collect();
	lines.pop();
	
	let result: Vec<&str> = lines
		.iter()
		.map(|number| if number.chars().eq(number.chars().rev()) { "yes" } else { "no" })
		.collect();
	
	println!("{}", result.join("\n"));
}

use std::io::{self, BufRead};

fn main() {
	println!("Day 1: The Tyranny of the Rocket Equation - Part 1");
	let stdin = io::stdin();

	let answer = stdin.lock()
		.lines()
		.map(|v| v.unwrap().parse::<i32>().unwrap())
		.fold(0, |a, v| a + (v / 3) - 2 );

	println!("Answer: {}", answer);
}
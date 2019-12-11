use std::io::{self, BufRead};

fn main() {
	println!("Day 1: The Tyranny of the Rocket Equation - Part 2");
	let stdin = io::stdin();

	let answer = stdin.lock()
		.lines()
		.map(|v| v.unwrap().parse::<i32>().unwrap())
		.fold(0, |a, v| {
			let mut total = 0;
			let mut fuel = v;
			loop {
				fuel = (fuel / 3) - 2;
				if fuel <= 0 {
					break
				}
				total += fuel
			}
			a + total
		} );

	println!("Answer: {}", answer);
}
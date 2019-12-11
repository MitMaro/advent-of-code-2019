use std::io::{self, BufRead};

fn main() {
	println!("Day 2: 1202 Program Alarm - Part 1");
	let stdin = io::stdin();

	let input = stdin.lock().lines().next().unwrap().unwrap();

	let mut memory: Vec<usize> = input
		.split(',')
		.map(|v| v.parse::<usize>().unwrap())
		.collect::<Vec<usize>>();

	memory[1] = 12;
	memory[2] = 2;

	let mut pc = 0;

	loop {
		let opcode = memory[pc];
		match opcode {
			1 => {
				match memory[pc+1..pc+4] {
					[i1, i2, r1] => {
						let a = memory[i1];
						let b = memory[i2];
						memory[r1] = a + b;
						pc += 4;
					},
					_ => panic!("Invalid input")
				}
			},
			2 => {
				match memory[pc+1..pc+4] {
					[i1, i2, r1] => {
						let a = memory[i1];
						let b = memory[i2];
						memory[r1] = a * b;
						pc += 4;
					},
					_ => panic!("Invalid input")
				}
			},
			99 => {break},
			_ => panic!("Unknown OP code: {}", opcode),
		}
	}

	println!("Answer: {}", memory[0]);
}
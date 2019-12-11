use std::io::{self, BufRead};

fn main() {
	println!("Day 2: 1202 Program Alarm - Part 2");
	let stdin = io::stdin();

	let input = stdin.lock().lines().next().unwrap().unwrap();

	let program = input
		.split(',')
		.map(|v| v.parse::<usize>().unwrap())
		.collect::<Vec<usize>>();

	let mut memory: Vec<usize> = vec![];

	let mut noun_value = 0;
	let mut verb_value = 0;

	let mut pc = 0;
	let mut opcode = 99;

	loop {
		if opcode == 99 {
			pc = 0;
			memory = program.to_vec();
			noun_value += 1;
			if noun_value > 99 {
				verb_value += 1;
				noun_value = 0;
			}
			memory[1] = noun_value;
			memory[2] = verb_value;
		}

		opcode = memory[pc];

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
			99 => {
				if memory[0] == 19690720 {
					println!("Answer: {}", 100 * noun_value + verb_value);
					break;
				}
			},
			_ => panic!("Unknown OP code: {}", opcode),
		}
	}
}
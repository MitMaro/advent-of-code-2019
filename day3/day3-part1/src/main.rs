use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
	X,
	Y
}

#[derive(Debug, Copy, Clone)]
struct LineSegment {
	direction: Direction,
	start: Point,
	end: Point,
}

fn new_line_from_string(s: &str) -> Vec<LineSegment> {
	let mut previous = Point {x: 0, y: 0};
	let mut position = Point {x: 0, y: 0};

	s.split(",").map(|v| {
		let dir;
		let direction = v.chars().next().unwrap();
		let value = v[1..].parse::<i32>().unwrap();
		match direction {
			'R' => {
				dir = Direction::X;
				position.x += value;
			},
			'L' => {
				dir = Direction::X;
				position.x -= value;
			},
			'D' => {
				dir = Direction::Y;
				position.y -= value;
			},
			'U' => {
				dir = Direction::Y;
				position.y += value
			},
			_ => panic!("Unknown input")
		}

		let (start, end) = match dir {
			Direction::X => {
				if position.x > previous.x {
					(previous, position)
				}
				else {
					(position, previous)
				}
			},
			Direction::Y => {
				if position.y > previous.y {
					(previous, position)
				}
				else {
					(position, previous)
				}
			},
		};

		let segment = LineSegment {
			direction: dir,
			start,
			end,
		};
		previous = Point {x: position.x, y: position.y};
		segment
	})
	.collect::<Vec<LineSegment>>()
}

fn main() {
	println!("Day 3: Crossed Wires - Part 1");
	let stdin = io::stdin();

	let mut lines = stdin.lock().lines();
	let wire1 = new_line_from_string(lines.next().unwrap().unwrap().as_str());
	let wire2 = new_line_from_string(lines.next().unwrap().unwrap().as_str());

	let mut intersections = vec![];

	for segment1 in wire1 {
		for segment2 in wire2.iter() {
			if segment1.direction == segment2.direction {
				continue;
			}

			match segment1.direction {
				Direction::X => {
					if segment1.start.x <= segment2.start.x && segment2.start.x <= segment1.end.x &&
					   segment2.start.y <= segment1.start.y && segment1.start.y <= segment2.end.y {
						intersections.push(Point {x: segment2.start.x, y: segment1.start.y});
					}
				},
				Direction::Y => {
					if segment1.start.y <= segment2.start.y &&  segment2.start.y <= segment1.end.y &&
					   segment2.start.x <= segment1.start.x && segment1.start.x <= segment2.end.x {
						intersections.push(Point {x: segment1.start.x, y: segment2.start.y});
					}
				},
			}
		}
	}

	if intersections[0].x == 0 && intersections[0].y == 0 {
		intersections.remove(0);
	}

	let mut max = intersections[0];

	for i in intersections {
		if (i.x.abs() + i.y.abs()) < (max.x.abs() + max.y.abs()) {
			max = i;
		}
	}

	eprintln!("{}", max.x.abs() + max.y.abs());
}
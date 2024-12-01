use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
	(part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
	let mut ours: Vec<u32> = Vec::new();
	let mut theirs: Vec<u32> = Vec::new();
	let input = input.lines();
	for l in input {
		let mut l = l.split_whitespace();
		ours.push(l.next().unwrap().parse().unwrap());
		theirs.push(l.next().unwrap().parse().unwrap());
	}
	ours.sort();
	theirs.sort();

	let mut total = 0;

	for (i, o) in ours.iter().enumerate() {
		total += o.abs_diff(theirs[i]);
	}
	
	total
}

fn part2(input: &str) -> impl Display {
	let mut ours: Vec<usize> = Vec::new();
	let mut theirs: Vec<usize> = Vec::new();
	let input = input.lines();
	for l in input {
		let mut l = l.split_whitespace();
		ours.push(l.next().unwrap().parse().unwrap());
		theirs.push(l.next().unwrap().parse().unwrap());
	}

	let mut total = 0;

	for o in ours {
		let count = theirs.iter().filter(|&&t| t == o).count();
		total += o * count;
	}
	
	total
}

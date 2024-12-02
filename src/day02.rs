use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
	(part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
	let input = input.lines();
	let mut safe_count = 0;
	for l in input {
		let mut split = l.split_whitespace().map(|i| i.parse::<isize>().unwrap()).peekable();
		let mut initial = split.next().unwrap();
		let next = split.peek().unwrap();
		let increasing = initial < *next;
		let mut safe = true;
		//println!("Increasing: {increasing}");
		for n in split {
			let diff = n - initial;
			//println!("Diff between {initial} and {n} is {diff}");
			initial = n;
			if diff < 0 {
				if increasing == false {
					if diff >= -3 && diff < 0 {
						//println!("safe");
						continue;
					}
				}
			} else {
				if increasing {
					if diff <= 3 && diff > 0 {
						//println!("safe");
						continue;
					}
				}
			}
			safe = false;
			//println!("not safe");
		}
		if safe {
			safe_count += 1;
		}
	}
	safe_count
}

fn part2(input: &str) -> impl Display {
	let input = input.lines();
	let mut safe_count = 0;
	for l in input {
		let split = l.split_whitespace().map(|i| i.parse::<isize>().unwrap()).collect::<Vec<_>>();
		for i in 0..=split.len() {
			let mut safe = true;
			let mut report = split.clone();
			if i > 0 {
				//println!("REMOVING LEVEL {i}");
				report.remove(i-1);
			}
			
			let mut last = report.remove(0);
			let increasing = last < report[0];

			if increasing {
				//println!("INCREASING");
			} else {
				//println!("DECREASING");
			}

			for n in report {
				let diff = n - last;
				//print!("Diff between {initial} and {n} is {diff} - ");

				//print!("{last} -> {n}: {diff} - ");
				
				if !increasing && diff < 0 && diff >= -3 {
					//safe step
					//println!("SAFE");
				} else if increasing && diff > 0 && diff <= 3 {
					//println!("SAFE");
					//safe step
				} else {
					//println!("UNSAFE");
					safe = false;
					break;
				}

				last = n;
			}

			if safe {
				//println!("REPORT IS SAFE, REMOVED LEVEL {i}\n");
				safe_count += 1;
				break;
			} else {
				//println!("REPORT IS UNSAFE\n");
			}
		}
	}
	safe_count
}

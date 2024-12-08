use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use itertools::Itertools;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
	let height = input.lines().count() as i32;
	let width = input.lines().next().unwrap().len() as i32;

	let mut frequencies = HashSet::new();
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some(((x as i32, y as i32), c))
                    }
                },
            )
        })
        .collect::<Vec<_>>();

	for &a in antennas.iter() {
		frequencies.insert(a.1);
	}

	let mut antinodes = HashSet::new();

	for freqeuncy in frequencies {
		let antenna_combos = antennas.iter().filter_map(|&(xy, f)| {if f == freqeuncy {Some(xy)} else {None}}).combinations(2);
		for combo in antenna_combos {
			let a = combo[0];
			let b = combo[1];

			let dx = b.0 - a.0;
			let dy = b.1 - a.1;

			antinodes.insert((a.0 - dx, a.1 - dy));
			antinodes.insert((b.0 + dx, b.1 + dy));
		}
	}

	antinodes.iter().filter(|&&(x, y)| x >= 0 && x < width && y >= 0 && y < height).count()
}

fn part2(input: &str) -> impl Display {
    let height = input.lines().count() as i32;
	let width = input.lines().next().unwrap().len() as i32;

	let mut frequencies = HashSet::new();
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some(((x as i32, y as i32), c))
                    }
                },
            )
        })
        .collect::<Vec<_>>();

	for &a in antennas.iter() {
		frequencies.insert(a.1);
	}

	let mut antinodes = HashSet::new();

	for freqeuncy in frequencies {
		let antenna_combos = antennas.iter().filter_map(|&(xy, f)| {if f == freqeuncy {Some(xy)} else {None}}).combinations(2);
		for combo in antenna_combos {
			let a = combo[0];
			let b = combo[1];

			let dx = b.0 - a.0;
			let dy = b.1 - a.1;

			antinodes.insert(a);
			antinodes.insert(b);

			let mut i = 1;
			loop {
				let x = a.0 - i * dx;
				let y = a.1 - i * dy;
				if !(x >= 0 && x < width && y >= 0 && y < height) {
					break;
				}

				antinodes.insert((x,y));
				i += 1;
			}

			let mut i = 1;
			loop {
				let x = b.0 + i * dx;
				let y = b.1 + i * dy;
				if !(x >= 0 && x < width && y >= 0 && y < height) {
					break;
				}

				antinodes.insert((x,y));
				i += 1;
			}

		}
	}

	antinodes.len()
}

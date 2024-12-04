use regex::Regex;
use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = re.captures_iter(input);
    let mut total = 0;
    for cap in caps {
        let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        total += a * b;
    }
    total
}

fn part2(input: &str) -> impl Display {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:do\(\))|(?:don't\(\))").unwrap();
    let caps = re.captures_iter(input);
    let mut enabled = true;
    let mut total = 0;
    for cap in caps {
        match cap.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                if enabled {
                    let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    total += a * b;
                }
            }
        }
    }
    total
}

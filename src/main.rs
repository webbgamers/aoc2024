use chrono::prelude::*;
use chrono_tz::EST;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use reqwest::StatusCode;
use std::env;
use std::fs;
use std::io::ErrorKind;

// Define each day as a module here
mod day01;
mod day02;
mod day03;
mod day04;
//mod day05;
//mod day06;
//mod day07;
//mod day08;
//mod day09;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
//mod day25;

fn main() {
    let (day, extras) = match {
        match env::args().nth(1).as_deref() {
            Some("extra") => match env::args().nth(2) {
                Some(day) => (Some(day), true),
                None => (None, true),
            },
            Some(day) => (Some(day.to_owned()), false),
            None => (None, false),
        }
    } {
        (None, extras) => {
            let time = Utc::now().with_timezone(&EST);
            if time.month() == 12 && time.day() <= 25 {
                (time.day(), extras)
            } else {
                println!("You must specify a day.");
                return;
            }
        }

        (Some(day), extras) => match day.parse::<u32>() {
            Ok(day) => (day, extras),
            Err(_) => {
                println!("Enter a vaild day.");
                return;
            }
        },
    };

    if day > 25 {
        println!("Enter a day between 1 and 25.");
        return;
    }

    let input = match get_input(&day) {
        Some(input) => input,
        None => {
            println!("Verify input file and try again.");
            return;
        }
    };

    // Extra fun stuff
    if extras {
        match day {
            //5 => day05::extra(input),
            day => println!("\nNo extras for day {day}.\n"),
        }
    } else {
        // Add each day to match statement here
        let (p1, p2) = match day {
            1 => day01::solve(input),
            2 => day02::solve(input),
            3 => day03::solve(input),
            4 => day04::solve(input),
            //5 => day05::solve(input),
            //6 => day06::solve(input),
            //7 => day07::solve(input),
            //8 => day08::solve(input),
            //9 => day09::solve(input),
            //10 => day10::solve(input),
            //11 => day11::solve(input),
            //12 => day12::solve(input),
            //13 => day13::solve(input),
            //14 => day14::solve(input),
            //15 => day15::solve(input),
            //16 => day16::solve(input),
            //17 => day17::solve(input),
            //18 => day18::solve(input),
            //19 => day19::solve(input),
            //20 => day20::solve(input),
            //21 => day21::solve(input),
            //22 => day22::solve(input),
            //23 => day23::solve(input),
            //24 => day24::solve(input),
            //25 => day25::solve(input),
            _ => (
                String::from("Not implemented yet"),
                String::from("Not implemented yet"),
            ),
        };
        println!("\nDay {} Part 1:", day);
        println!("{}\n", p1);
        println!("Day {} Part 2:", day);
        println!("{}\n", p2);
    }
}

pub fn get_input(day: &u32) -> Option<String> {
    let fp = format!("input/day{:02}.txt", day);
    match fs::read_to_string(&fp) {
        Ok(input) => Some(input),
        Err(error) => {
            if let ErrorKind::NotFound = error.kind() {
                println!("Input file not found, attempting to download.");
                let token = if let Some(token) = get_aoc_token() {
                    token
                } else {
                    println!("Save your AOC session token to './aoc_token' or to the AOC_TOKEN environment variable.");
                    return None;
                };
                let c = Client::new();
                match c
                    .get(format!("https://adventofcode.com/2024/day/{}/input", day))
                    .header(COOKIE, format!("session={}", token))
                    .send()
                {
                    Ok(res) => match res.status() {
                        StatusCode::OK => match res.text() {
                            Ok(text) => {
                                match fs::exists("./input") {
                                    Ok(false) => {
                                        if let Err(err) = fs::create_dir("./input") {
                                            println!("Couldn't create './input' directory: {err}")
                                        }
                                    },
                                    Err(err) => {
                                        println!("Couldn't check './input' directory: {err}");
                                        return None;
                                    },
                                    _ => {},
                                }
                                let fp = format!("./input/day{:02}.txt", day);
                                if let Err(err) = fs::write(&fp, &text) {
                                    println!("Couldn't save input to '{}': {}", fp, err);
                                } else {
                                    println!("Saved input to {}", fp);
                                    return Some(text);
                                }
                            }
                            Err(err) => println!("Failed to parse response: {}", err),
                        },
                        StatusCode::NOT_FOUND => {
                            println!("Input for day {} not found, might be too early.", day)
                        }
                        code => println!("Unexpected response: {}", code),
                    },
                    Err(err) => {
                        println!("Failed to download input: {}", err);
                    }
                }

                None
            } else {
                println!("Failed to load input file '{}': {}", &fp, error);
                None
            }
        }
    }
}

pub fn get_aoc_token() -> Option<String> {
    if let Ok(token) = fs::read_to_string("./aoc_token") {
        Some(token)
    } else if let Some((_, v)) = env::vars().find(|(k, _)| k == "AOC_TOKEN") {
        Some(v)
    } else {
        println!("Could not find AOC session token.");
        None
    }
}

# Advent of Code 2024
Here we go again...
This is my testing system and (eventually) solutions for [AOC 2024](https://adventofcode.com/2024). Doing everything in Rust for a 4th year in a row because why not?

## How to see my solutions
`cargo run [day]` (If no day is provided during competition time, the current date is used)

## Extra stuff
To run any extra stuff for a particular day you can use `cargo run extra [day]`. I am using this for vizualizations but really anything could be done.

## How to use testing system
Each day gets a src/day##.rs file with the definitions shown in src/template.rs and have corresponding lines in src/main.rs uncommented. For extras, a public extra function needs to be defined for the particular day and the function needs to be added to the extras match statement in src/main.rs.

## Automatic input downloading
If no input is found in input/day##.txt, the input for the selected day will be downloaded. There must be an AOC token placed in the aoc_token file or in the AOC_TOKEN environment vairable. You can find this in `session` cookie once logged into the website.

## Dependencies 
 - [Rust Toolchain](https://www.rust-lang.org/tools/install)
 - Will to live (optional)

## Coming Soon
Automatic answer submittal right from the CLI! (didn't happen last year or the year before that so we'll see)

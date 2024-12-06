use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut y = map.iter().position(|row| row.contains(&'^')).unwrap();
    let mut x = map[y].iter().position(|&c| c == '^').unwrap();
    let mut visited = Vec::new();
    let mut direction = 0;
    loop {
        if !visited.contains(&(y, x)) {
            visited.push((y, x))
        };
        let next_pos = match direction {
            0 => (y as i32 - 1, x as i32),
            1 => (y as i32, x as i32 + 1),
            2 => (y as i32 + 1, x as i32),
            3 => (y as i32, x as i32 - 1),
            _ => panic!("Invaid direction"),
        };

        if next_pos.0 < 0 || next_pos.0 >= height || next_pos.1 < 0 || next_pos.1 >= width {
            break;
        }

        if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction = (direction + 1) % 4;
        } else {
            y = next_pos.0 as usize;
            x = next_pos.1 as usize;
        }

        /*for y in 0..height as usize {
            for x in 0..width as usize {
                if visited.contains(&(y, x)) {
                    print!("X");
                } else {
                    print!("{}", map[y][x]);
                }
            }
            println!();
        }*/
    }

    return visited.len();
}

fn part2(input: &str) -> impl Display {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_y = map.iter().position(|row| row.contains(&'^')).unwrap();
    let start_x = map[start_y].iter().position(|&c| c == '^').unwrap();

    let mut y = start_y;
    let mut x = start_x;
    let mut visited = Vec::new();
    let mut direction = 0;
    loop {
        if !visited.contains(&(y, x)) {
            visited.push((y, x))
        };
        let next_pos = match direction {
            0 => (y as i32 - 1, x as i32),
            1 => (y as i32, x as i32 + 1),
            2 => (y as i32 + 1, x as i32),
            3 => (y as i32, x as i32 - 1),
            _ => panic!("Invaid direction"),
        };

        if next_pos.0 < 0 || next_pos.0 >= height || next_pos.1 < 0 || next_pos.1 >= width {
            break;
        }

        if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            direction = (direction + 1) % 4;
        } else {
            y = next_pos.0 as usize;
            x = next_pos.1 as usize;
        }
    }

    let mut count = 0;

    for (y, x) in visited {
        if y != start_y || x != start_x {
            let mut map = map.clone();
            map[y][x] = 'O';

            let mut y = start_y;
            let mut x = start_x;

            let mut visited = Vec::new();
            let mut direction = 0;
            loop {
                if !visited.contains(&(y, x, direction)) {
                    visited.push((y, x, direction))
                } else {
                    //println!("Found valid position!");
                    count += 1;
                    break;
                };
                let next_pos = match direction {
                    0 => (y as i32 - 1, x as i32),
                    1 => (y as i32, x as i32 + 1),
                    2 => (y as i32 + 1, x as i32),
                    3 => (y as i32, x as i32 - 1),
                    _ => panic!("Invaid direction"),
                };

                if next_pos.0 < 0 || next_pos.0 >= height || next_pos.1 < 0 || next_pos.1 >= width {
                    //println!("Found invalid position!");
                    break;
                }

                if map[next_pos.0 as usize][next_pos.1 as usize] == '#'
                    || map[next_pos.0 as usize][next_pos.1 as usize] == 'O'
                {
                    direction = (direction + 1) % 4;
                } else {
                    y = next_pos.0 as usize;
                    x = next_pos.1 as usize;
                }
            }
        }
    }

    return count;
}

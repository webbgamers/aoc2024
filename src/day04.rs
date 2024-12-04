use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn xy_to_i(x: isize, y: isize, w: usize, h: usize) -> Option<usize> {
    if x < 0 || x >= w as isize || y < 0 || y >= h as isize {
        return None;
    } else {
        Some(y as usize * w + x as usize)
    }
}

fn i_to_xy(i: usize, w: usize) -> (usize, usize) {
    (i % w, i / w)
}

fn part1(input: &str) -> impl Display {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let key = ['X', 'M', 'A', 'S'];

    let mut count = 0;

    for (i, _) in input.iter().enumerate() {
        // Left
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(x as isize - d as isize, y as isize, width, height) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
        // Up
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(x as isize, y as isize - d as isize, width, height) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
        // Right
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(x as isize + d as isize, y as isize, width, height) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
        // Down
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(x as isize, y as isize + d as isize, width, height) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
        // Up-right
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(
                x as isize + d as isize,
                y as isize - d as isize,
                width,
                height,
            ) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
        // Down-right
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(
                x as isize + d as isize,
                y as isize + d as isize,
                width,
                height,
            ) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
        // Up-left
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(
                x as isize - d as isize,
                y as isize - d as isize,
                width,
                height,
            ) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
        // Down-left
        for d in 0..=3 {
            let (x, y) = i_to_xy(i, width);
            if let Some(index) = xy_to_i(
                x as isize - d as isize,
                y as isize + d as isize,
                width,
                height,
            ) {
                if input[index] != key[d] {
                    break;
                }
                if d == 3 {
                    count += 1;
                }
            } else {
                break;
            }
        }
    }
    count
}

fn part2(input: &str) -> impl Display {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let mut count = 0;

    for (i, &c) in input.iter().enumerate() {
        let (x, y) = i_to_xy(i, width);
        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
            continue;
        }
        let x = x as isize;
        let y = y as isize;

        // M's-up
        if c == 'A'
            && input[xy_to_i(x - 1, y - 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x + 1, y - 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x - 1, y + 1, width, height).unwrap()] == 'S'
            && input[xy_to_i(x + 1, y + 1, width, height).unwrap()] == 'S'
        {
            count += 1;
        }

        // M's-right
        if c == 'A'
            && input[xy_to_i(x + 1, y - 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x + 1, y + 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x - 1, y - 1, width, height).unwrap()] == 'S'
            && input[xy_to_i(x - 1, y + 1, width, height).unwrap()] == 'S'
        {
            count += 1;
        }

        // M's-down
        if c == 'A'
            && input[xy_to_i(x - 1, y + 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x + 1, y + 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x - 1, y - 1, width, height).unwrap()] == 'S'
            && input[xy_to_i(x + 1, y - 1, width, height).unwrap()] == 'S'
        {
            count += 1;
        }

        // M's-left
        if c == 'A'
            && input[xy_to_i(x - 1, y - 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x - 1, y + 1, width, height).unwrap()] == 'M'
            && input[xy_to_i(x + 1, y - 1, width, height).unwrap()] == 'S'
            && input[xy_to_i(x + 1, y + 1, width, height).unwrap()] == 'S'
        {
            count += 1;
        }
    }
    count
}

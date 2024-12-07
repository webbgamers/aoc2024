use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let equations = input.lines().map(|l| {
        l.split(|c: char| c == ':' || c.is_whitespace())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut total = 0;
    for equation in equations {
        let test_val = equation[0];
        let mut operators = vec!['+'; equation.len() - 2];

        loop {
            // Evaluate
            let mut result = equation[1];
            for (i, op) in operators.iter().enumerate() {
                match *op {
                    '+' => result += equation[i + 2],
                    '*' => result *= equation[i + 2],
                    _ => panic!("Invalid operation"),
                }
            }

            if result == test_val {
                total += test_val;
                break;
            }

            // Permute operators
            if operators.iter().all(|&c| c == '*') {
                break;
            }
            let mut i = operators.len() - 1;
            loop {
                if operators[i] == '+' {
                    operators[i] = '*';
                    break;
                } else {
                    operators[i] = '+';
                    if i == 0 {
                        break;
                    }
                    i -= 1;
                }
            }
        }
    }

    total
}

fn part2(input: &str) -> impl Display {
    let equations = input.lines().map(|l| {
        l.split(|c: char| c == ':' || c.is_whitespace())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut total = 0;
    for equation in equations {
        let test_val = equation[0];
        let mut operators = vec!['+'; equation.len() - 2];

        loop {
            // Evaluate
            let mut result = equation[1];
            for (i, op) in operators.iter().enumerate() {
                match *op {
                    '+' => result += equation[i + 2],
                    '*' => result *= equation[i + 2],
                    '|' => {
                        let mut a = result.to_string();
                        let b = equation[i + 2].to_string();
                        a.push_str(&b);
                        result = a.parse::<usize>().unwrap();
                    }
                    _ => panic!("Invalid operation"),
                }
            }

            if result == test_val {
                total += test_val;
                break;
            }

            // Permute operators
            if operators.iter().all(|&c| c == '|') {
                break;
            }
            let mut i = operators.len() - 1;
            loop {
                match operators[i] {
                    '+' => {
                        operators[i] = '*';
                        break;
                    }
                    '*' => {
                        operators[i] = '|';
                        break;
                    }
                    _ => {
                        operators[i] = '+';
                        if i == 0 {
                            break;
                        }
                        i -= 1;
                    }
                }
            }
        }
    }

    total
}

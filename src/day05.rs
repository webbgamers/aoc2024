use std::fmt::Display;

pub fn solve(input: String) -> (String, String) {
    (part1(&input).to_string(), part2(&input).to_string())
}

fn part1(input: &str) -> impl Display {
    let mut input = input.split("\n\n");
    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split('|')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut updates = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    for update in updates.iter_mut() {
        //loop {
        //println!("{:?}", update);
        let mut updated = false;
        for rule in rules.iter() {
            //println!("Checking against {:?}", rule);
            if update.contains(&rule[0]) && update.contains(&rule[1]) {
                let i1 = update.iter().position(|&page| page == rule[0]).unwrap();
                let i2 = update.iter().position(|&page| page == rule[1]).unwrap();
                //println!("Update contains both at index {i1} and {i2}");
                if i1 > i2 {
                    //println!("Swapped {} and {}", rule[0], rule[1]);
                    update[i2] = rule[0];
                    update[i1] = rule[1];
                    updated = true;
                }
            }
        }
        if !updated {
            total += update[update.len() / 2];
        }
        //}
    }

    //for update in updates {
    //	total += update[update.len()/2];
    //}

    total
}

fn part2(input: &str) -> impl Display {
    let p1 = part1(input).to_string().parse::<usize>().unwrap();

    let mut input = input.split("\n\n");
    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split('|')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut updates = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    for update in updates.iter_mut() {
        loop {
            //println!("{:?}", update);
            let mut updated = false;
            for rule in rules.iter() {
                //println!("Checking against {:?}", rule);
                if update.contains(&rule[0]) && update.contains(&rule[1]) {
                    let i1 = update.iter().position(|&page| page == rule[0]).unwrap();
                    let i2 = update.iter().position(|&page| page == rule[1]).unwrap();
                    //println!("Update contains both at index {i1} and {i2}");
                    if i1 > i2 {
                        //println!("Swapped {} and {}", rule[0], rule[1]);
                        update[i2] = rule[0];
                        update[i1] = rule[1];
                        updated = true;
                    }
                }
            }
            if !updated {
                break;
            }
        }
    }

    for update in updates {
        total += update[update.len() / 2];
    }

    total - p1
}

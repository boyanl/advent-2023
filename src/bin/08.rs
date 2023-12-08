use std::{cmp, collections::HashMap, io::stdin};

use scanf::sscanf;

type DestinationsMap = HashMap<String, (String, String)>;
fn read_input() -> (String, DestinationsMap) {
    let mut instructions = String::new();

    stdin()
        .read_line(&mut instructions)
        .expect("can't read instructions");

    instructions = String::from(&instructions[..instructions.len() - 1]);

    let mut destinations = HashMap::new();

    for line in stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            continue;
        }

        let (mut from, mut left, mut right) = (String::new(), String::new(), String::new());
        if sscanf!(&line, "{} = ({}, {})", from, left, right).is_ok() {
            destinations.insert(from.clone(), (left.clone(), right.clone()));
        }
    }

    (instructions, destinations)
}

fn advance_one<'a>(current: &str, instruction: char, destinations: &'a DestinationsMap) -> &'a str {
    return match instruction {
        'L' => &destinations[current].0,
        'R' => &destinations[current].1,
        _ => todo!(),
    };
}

fn part_one() {
    let (instructions, destinations) = read_input();

    let destination = "ZZZ";
    let mut current = "AAA";

    let mut steps = 0;

    'out: while current != destination {
        for instruction in instructions.chars() {
            current = advance_one(&current, instruction, &destinations);
            steps += 1;

            if current == destination {
                break 'out;
            }
        }
    }

    println!("{steps}")
}

fn is_final(state: &str) -> bool {
    return state.ends_with("Z");
}

fn find_cycle_len(state: &str, instructions: &str, destinations: &DestinationsMap) -> (i32, i32) {
    let mut current = state;
    let mut steps = 0;

    let mut prev_step = -1;
    let mut first_step = -1;
    let mut first_final_state: &str = "";
    let mut steps_vec = Vec::new();

    'out: loop {
        for instruction in instructions.chars() {
            current = advance_one(current, instruction, destinations);
            steps += 1;

            if is_final(current) {
                if prev_step == -1 {
                    first_step = steps;
                    first_final_state = current;
                } else {
                    steps_vec.push(steps - prev_step);

                    if current == first_final_state {
                        break 'out;
                    }
                }
                prev_step = steps;
            }
        }
    }

    assert!(steps_vec.len() == 1);
    let cycle_len = steps_vec[0];
    return (first_step, cycle_len);
}

fn gcd(i1: i64, i2: i64) -> i64 {
    let (mut d1, mut d2) = (cmp::max(i1, i2), cmp::min(i1, i2));

    while d2 != 0 {
        let rem = d1 % d2;
        d1 = d2;
        d2 = rem;
    }

    d1
}

fn lcm(i1: i64, i2: i64) -> i64 {
    (i1 / gcd(i1, i2)) * i2
}

fn part_two() {
    let (instructions, destinations) = read_input();

    let starts = destinations
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.as_str())
        .collect::<Vec<_>>();

    let cycle_lens = starts
        .iter()
        .map(|&start| {
            let (start, cycle_len) = find_cycle_len(start, &instructions, &destinations);

            assert_eq!(start, cycle_len);
            return cycle_len as i64;
        })
        .collect::<Vec<_>>();

    let result = cycle_lens
        .iter()
        .map(|x| *x)
        .reduce(|a, b| lcm(a, b))
        .unwrap();

    println!("{result}");
}

fn main() {
    part_two();
}

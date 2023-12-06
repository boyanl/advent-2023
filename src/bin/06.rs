use std::{cmp, io::stdin};

use scanf::sscanf;

#[derive(Clone, Copy, Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn read_input() -> Vec<Race> {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut times_str = String::new();
        let mut distance_str = String::new();
        if sscanf!(&line, "Time: {}", times_str).is_ok() {
            times = times_str
                .split_ascii_whitespace()
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
        } else if sscanf!(&line, "Distance: {}", distance_str).is_ok() {
            distances = distance_str
                .split_ascii_whitespace()
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
        }
    }

    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }

    races
}

fn ways_to_win(r: Race) -> i64 {
    let (d, t) = (r.distance, r.time);
    let discr = (t * t - 4 * d) as f64;
    let (x1, x2) = (
        (((t as f64) - discr.sqrt()) / 2.0),
        (((t as f64) + discr.sqrt()) / 2.0),
    );

    let x1 = if x1 == x1.ceil() {
        (x1.ceil() as i64) + 1
    } else {
        x1.ceil() as i64
    };

    let x2 = if x2 == x2.floor() {
        (x2.floor() as i64) - 1
    } else {
        x2.floor() as i64
    };

    let (t1, t2) = (cmp::max(x1, 0), cmp::min(x2, r.time));

    return t2 - t1 + 1;
}

fn part_one() {
    let races = read_input();
    let result = races.iter().map(|r| ways_to_win(*r)).product::<i64>();

    println!("{result}");
}

fn read_input_2() -> Race {
    let mut time = 0i64;
    let mut distance = 0i64;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut times_str = String::new();
        let mut distance_str = String::new();
        if sscanf!(&line, "Time: {}", times_str).is_ok() {
            time = times_str
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
                .join("")
                .parse::<i64>()
                .unwrap();
        } else if sscanf!(&line, "Distance: {}", distance_str).is_ok() {
            distance = distance_str
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
                .join("")
                .parse::<i64>()
                .unwrap();
        }
    }

    Race {
        time: time,
        distance: distance,
    }
}

fn part_two() {
    let race = read_input_2();
    let result = ways_to_win(race);

    println!("{result}");
}

fn main() {
    part_two();
}

use std::cmp::max;
use std::io::stdin;

use scanf::sscanf;

#[derive(Debug, Clone, Copy)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Clone)]
struct Game {
    id: i32,
    sets: Vec<CubeSet>,
}

fn parse_set(s: &str) -> CubeSet {
    let mut result: CubeSet = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for part in s.split(", ") {
        let split = part.split_ascii_whitespace().collect::<Vec<_>>();
        let amount = split[0].parse::<i32>().expect("expected an int for amount");
        let color = split[1];
        match color {
            "red" => result.red = amount,
            "green" => result.green = amount,
            "blue" => result.blue = amount,
            _ => todo!("unexpected color {color}"),
        }
    }

    return result;
}

fn parse_input() -> Vec<Game> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut id: i32 = 0;
        let mut bag_contents = String::new();
        if sscanf!(&line, "Game {i32}: {}", id, bag_contents).is_ok() {
            let sets = bag_contents.split("; ").map(parse_set).collect::<Vec<_>>();
            result.push(Game { id: id, sets: sets });
        }
    }

    return result;
}

fn possible(g: &Game, available: CubeSet) -> bool {
    return g.sets.iter().all(|&set| {
        set.red <= available.red && set.green <= available.green && set.blue <= available.blue
    });
}

fn part_one() {
    let input = parse_input();
    let available = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = input
        .iter()
        .filter(|&game| possible(game, available))
        .map(|g| g.id)
        .sum::<i32>();

    println!("{result}")
}

fn power(cubeset: CubeSet) -> i32 {
    return cubeset.red * cubeset.green * cubeset.blue;
}

fn min_required(game: &Game) -> CubeSet {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for set in &game.sets {
        red = max(set.red, red);
        green = max(set.green, green);
        blue = max(set.blue, blue);
    }

    return CubeSet {
        red: red,
        green: green,
        blue: blue,
    };
}

fn part_two() {
    let input = parse_input();

    let result = input
        .iter()
        .map(|game| power(min_required(game)))
        .sum::<i32>();

    println!("{result}");
}

fn main() {
    part_two();
}

use std::io::stdin;

type Map = Vec<Vec<char>>;

fn read_input() -> Vec<Map> {
    let mut result = Vec::new();
    let mut current_map = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        if line.is_empty() {
            if !current_map.is_empty() {
                result.push(current_map);
                current_map = Vec::new();
            }
            continue;
        }
        let chars = line.chars().collect::<Vec<_>>();
        current_map.push(chars);
    }

    if !current_map.is_empty() {
        result.push(current_map);
    }

    result
}

fn differences_if_mirror_col(col: usize, map: &Vec<Vec<char>>) -> i32 {
    let (mut before, mut after) = (col - 1, col);
    let mut diffs = 0;

    loop {
        for row in 0..map.len() {
            if map[row][before] != map[row][after] {
                diffs += 1;
            }
        }
        if before == 0 || after == map[0].len() - 1 {
            break;
        }
        before -= 1;
        after += 1;
    }
    return diffs;
}

fn differences_if_mirror_row(row: usize, map: &Vec<Vec<char>>) -> i32 {
    let (mut before, mut after) = (row - 1, row);
    let mut diffs = 0;

    loop {
        for col in 0..map[0].len() {
            if map[before][col] != map[after][col] {
                diffs += 1;
            }
        }
        if before == 0 || after == map.len() - 1 {
            break;
        }
        before -= 1;
        after += 1;
    }
    return diffs;
}

fn find_symmetry_score(m: &Map, diffs: i32) -> i32 {
    let mut result = 0;
    for col in 1..m[0].len() {
        if differences_if_mirror_col(col, &m) == diffs {
            result += col as i32;
        }
    }

    for row in 1..m.len() {
        if differences_if_mirror_row(row, &m) == diffs {
            result += 100 * row as i32;
        }
    }

    result
}

fn part_one() {
    let input = read_input();

    let result = input
        .iter()
        .map(|part| find_symmetry_score(part, 0))
        .sum::<i32>();

    println!("{result}");
}

fn part_two() {
    let input = read_input();

    let result = input
        .iter()
        .map(|part| find_symmetry_score(part, 1))
        .sum::<i32>();

    println!("{result}");
}

fn main() {
    part_two();
}

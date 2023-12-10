use std::io::stdin;

fn predict_next(seq: &Vec<i32>) -> i32 {
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    let mut current = seq.clone();

    loop {
        diffs.push(current.clone());
        let mut current_diffs = Vec::new();

        if current.len() <= 1 {
            break;
        }

        for i in 0..current.len() - 1 {
            current_diffs.push(current[i + 1] - current[i]);
        }

        if current_diffs.iter().all(|x| *x == current_diffs[0]) {
            let mut result = current_diffs[0];
            for diffs_row in diffs.iter().rev() {
                result = diffs_row.last().unwrap() + result;
            }
            return result;
        }

        current = current_diffs.clone();
    }

    -1
}

fn part_one() {
    let mut result = 0;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let seq = line
            .split_ascii_whitespace()
            .map(|part| part.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        result += predict_next(&seq);
    }

    println!("{result}");
}

fn part_two() {
    let mut result = 0;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let seq = line
            .split_ascii_whitespace()
            .map(|part| part.parse::<i32>().unwrap())
            .rev()
            .collect::<Vec<_>>();

        result += predict_next(&seq);
    }

    println!("{result}");
}

fn main() {
    part_two();
}

use memoize::memoize;
use scanf::sscanf;
use std::{io::stdin, vec};

#[memoize]
fn valid_combinations(line: String, groups: Vec<i32>, in_group: bool) -> i64 {
    if line.is_empty() {
        let ok = groups.len() == 0 || (groups.len() == 1 && groups[0] == 0);
        return if ok { 1 } else { 0 };
    }

    let c = line.chars().nth(0).unwrap();
    let rest = line[1..].to_string();

    match c {
        '.' => {
            if in_group {
                if groups[0] != 0 {
                    return 0;
                }

                return valid_combinations(rest, groups[1..].to_vec(), false);
            }
            return valid_combinations(rest, groups, false);
        }
        '#' => {
            if groups.len() > 0 && groups[0] > 0 {
                let mut new_groups = groups.clone();
                new_groups[0] -= 1;

                return valid_combinations(rest, new_groups, true);
            }

            return 0;
        }
        '?' => {
            let line1 = ".".to_string() + rest.as_str();
            let line2 = "#".to_string() + rest.as_str();
            let res = valid_combinations(line1, groups.clone(), in_group)
                + valid_combinations(line2, groups, in_group);

            return res;
        }
        _ => todo!(),
    }
}

fn possibilities(line: &str, groups: &Vec<i32>) -> i64 {
    return valid_combinations(line.to_string(), groups.clone(), false);
}

fn possibilities_dp(line: &str, groups: &Vec<i32>) -> i64 {
    // Surround the input and the desired pattern with "."
    // to account for leading/trailing "."-s (those don't change the groups)
    let desired = ".".to_string()
        + &groups
            .iter()
            .map(|n| "#".repeat(*n as usize))
            .collect::<Vec<_>>()
            .join(".")
        + ".";

    let line = ".".to_string() + line + ".";

    let n = line.len();
    let m = desired.len();

    // how many ways we can match line[..i] to desired[..j]
    let mut matches = vec![vec![0; m + 1]; n + 1];
    // If both are empty, they match
    matches[0][0] = 1;

    let (chars1, chars2) = (line.as_bytes(), desired.as_bytes());

    for i in 1..=n {
        for j in 1..=m {
            let (have, want) = (chars1[i - 1] as char, chars2[j - 1] as char);
            let matching = have == want || have == '?';

            if matching && want == '#' {
                matches[i][j] = matches[i - 1][j - 1];
            }
            if matching && want == '.' {
                // The '.' in `want` can match one or more '.'-s in `have`, so we can either "consume" it
                // or not
                matches[i][j] += matches[i - 1][j - 1] + matches[i - 1][j];
            }
        }
    }

    matches[n][m]
}

fn part_one() {
    let mut result = 0;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut records, mut groups_str) = (String::new(), String::new());
        if sscanf!(&line, "{} {}", records, groups_str).is_ok() {
            let groups = groups_str
                .split(",")
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            result += possibilities_dp(&records, &groups);
        }
    }

    println!("{result}");
}

fn part_two() {
    let mut result = 0;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut records, mut groups_str) = (String::new(), String::new());
        if sscanf!(&line, "{} {}", records, groups_str).is_ok() {
            let times = 5;
            let records_new: String = vec![records; times].join("?").into();
            let groups_new: String = vec![groups_str; times].join(",").into();

            let groups = groups_new
                .split(",")
                .map(|part| part.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            result += possibilities_dp(&records_new, &groups);
        }
    }

    println!("{result}");
}

fn main() {
    part_two();
}

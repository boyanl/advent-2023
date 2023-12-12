use memoize::memoize;
use scanf::sscanf;
use std::io::stdin;

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

    return 0;
}

fn possibilities(line: &str, groups: &Vec<i32>) -> i64 {
    return valid_combinations(line.to_string(), groups.clone(), false);
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

            let p = possibilities(&records, &groups);
            result += p;
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

            result += possibilities(&records_new, &groups);
        }
    }

    println!("{result}");
}

fn main() {
    part_two();
}

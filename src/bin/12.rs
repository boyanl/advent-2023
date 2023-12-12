use scanf::sscanf;
use std::{collections::HashMap, io::stdin};

fn cache_val(state: (String, Vec<i32>), v: i64, map: &mut HashMap<(String, Vec<i32>), i64>) -> i64 {
    map.insert(state, v);
    return v;
}

fn valid_combinations(
    line: &str,
    groups: &[i32],
    in_group: bool,
    cache: &mut HashMap<(String, Vec<i32>), i64>,
) -> i64 {
    let state = (line.to_string(), groups.to_vec());

    let cached = cache.get(&state);
    if let Some(n) = cached {
        return *n;
    }

    if line.is_empty() {
        let ok = groups.len() == 0 || (groups.len() == 1 && groups[0] == 0);
        return cache_val(state, if ok { 1 } else { 0 }, cache);
    }

    let c = line.chars().nth(0).unwrap();
    let rest = &line[1..];

    if c == '.' {
        if in_group {
            if groups[0] != 0 {
                return 0;
            }

            return cache_val(
                state,
                valid_combinations(rest, &groups[1..], false, cache),
                cache,
            );
        }

        return cache_val(state, valid_combinations(rest, groups, false, cache), cache);
    } else if c == '#' {
        if groups.len() > 0 && groups[0] > 0 {
            let mut new_groups = groups.to_vec();
            new_groups[0] -= 1;

            return cache_val(
                state,
                valid_combinations(rest, &new_groups, true, cache),
                cache,
            );
        }

        return cache_val(state, 0, cache);
    } else if c == '?' {
        let line1 = ".".to_string() + rest;
        let line2 = "#".to_string() + rest;
        let res = valid_combinations(&line1, groups, in_group, cache)
            + valid_combinations(&line2, groups, in_group, cache);

        return cache_val(state, res, cache);
    }

    return cache_val(state, 0, cache);
}

fn possibilities(line: &str, groups: &Vec<i32>) -> i64 {
    return valid_combinations(line, groups, false, &mut HashMap::new());
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
            println!("{records}, {:?} = {p} possibilities", groups);
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

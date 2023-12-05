use std::{
    cmp,
    collections::{HashMap, VecDeque},
    io::stdin,
};

use scanf::sscanf;

#[derive(Debug)]
struct IntervalMapping {
    src: i64,
    len: i64,
    dest: i64,
}

type Maps = HashMap<String, (String, Vec<IntervalMapping>)>;

fn find_mapping_simple(n: i64, mapping: &Vec<IntervalMapping>) -> i64 {
    for interval in mapping {
        if interval.src <= n && n < interval.src + interval.len {
            return n - interval.src + interval.dest;
        }
    }
    return n;
}

fn find_mapping(maps: &Maps, n: i64, from: &str, to: &str) -> i64 {
    let mut q = VecDeque::new();
    q.push_back((from, n));
    while !q.is_empty() {
        let (stage, n) = q.pop_front().unwrap();
        if stage == to {
            return n;
        }

        let (next, map) = maps.get(stage).unwrap();
        q.push_back((next, find_mapping_simple(n, map)));
    }

    -1
}

fn read_input() -> (Vec<i64>, Maps) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Maps = HashMap::new();

    let mut in_map = false;
    let (mut from, mut to) = (String::new(), String::new());
    let mut current_map = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut seeds_str = String::new();
        if sscanf!(&line, "seeds: {}", seeds_str).is_ok() {
            seeds = seeds_str
                .split_ascii_whitespace()
                .map(|part| part.parse::<i64>().expect("need a number"))
                .collect::<Vec<_>>();
        } else if sscanf!(&line, "{}-to-{} map:", from, to).is_ok() {
            in_map = true;
        } else if line.is_empty() {
            in_map = false;
            maps.insert(from.clone(), (to.clone(), current_map));
            current_map = Vec::new();
        } else if in_map {
            let (mut dest, mut src, mut len) = (0, 0, 0);
            if sscanf!(&line, "{i64} {i64} {i64}", dest, src, len).is_ok() {
                current_map.push(IntervalMapping {
                    src: src,
                    len: len,
                    dest: dest,
                });
            }
        }
    }

    if !current_map.is_empty() {
        maps.insert(from.clone(), (to.clone(), current_map));
    }

    (seeds, maps)
}

fn part_one() {
    let (seeds, maps) = read_input();

    let result = seeds
        .iter()
        .map(|seed| find_mapping(&maps, *seed, "seed", "location"))
        .min()
        .expect("should have at least 1 position");
    println!("{result}");
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

fn is_empty(i: Interval) -> bool {
    return i.start >= i.end;
}

fn len(i: Interval) -> i64 {
    return i.end - i.start;
}

fn intersection(i1: Interval, i2: Interval) -> Interval {
    return Interval {
        start: cmp::max(i1.start, i2.start),
        end: cmp::min(i1.end, i2.end),
    };
}

fn find_mapping_ranges_simple(
    ranges: &Vec<Interval>,
    mapping: &Vec<IntervalMapping>,
) -> Vec<Interval> {
    // Split ranges into pieces that are fully contained within the mapping intervals
    // So we don't have to handle partially overlapping intervals later
    let mut ranges = ranges.clone();
    for mapping_interval in mapping {
        let mut new_ranges = Vec::new();
        for range in &ranges {
            let interval2 = Interval {
                start: mapping_interval.src,
                end: mapping_interval.src + mapping_interval.len,
            };

            let isect = intersection(*range, interval2);

            if !is_empty(isect) {
                if isect.start > range.start {
                    new_ranges.push(Interval {
                        start: range.start,
                        end: isect.start,
                    });
                }

                new_ranges.push(isect);

                if isect.end < range.end {
                    new_ranges.push(Interval {
                        start: isect.end,
                        end: range.end,
                    })
                }
            } else {
                new_ranges.push(*range);
            }
        }
        ranges = new_ranges;
    }

    let mut result = Vec::new();
    for range in &ranges {
        let mut found = false;
        for mapping_interval in mapping {
            let interval2 = Interval {
                start: mapping_interval.src,
                end: mapping_interval.src + mapping_interval.len,
            };

            let isect = intersection(*range, interval2);

            if !is_empty(isect) {
                let start = mapping_interval.dest + (isect.start - mapping_interval.src);
                result.push(Interval {
                    start: start,
                    end: start + (isect.end - isect.start),
                });
                found = true;
                break;
            }
        }

        if !found {
            result.push(*range);
        }
    }

    result.sort_by_key(|i1| i1.start);

    return result;
}

fn find_mapping_ranges(maps: &Maps, ranges: &Vec<Interval>, from: &str, to: &str) -> Vec<Interval> {
    let mut q: VecDeque<(&str, Vec<Interval>)> = VecDeque::new();
    q.push_back((from, ranges.clone()));

    while !q.is_empty() {
        let (stage, ranges) = q.pop_front().unwrap();
        if stage == to {
            return ranges.clone();
        }

        let (next, map) = maps.get(stage).unwrap();
        q.push_back((next, find_mapping_ranges_simple(&ranges, map)));
    }

    return Vec::new();
}

fn part_two() {
    let (seeds, maps) = read_input();
    let mut seed_intervals = Vec::new();

    let mut i = 0;
    while i < seeds.len() {
        let (start, len) = (seeds[i], seeds[i + 1]);
        seed_intervals.push(Interval {
            start: start,
            end: start + len,
        });

        i += 2;
    }

    let result_intervals = find_mapping_ranges(&maps, &seed_intervals, "seed", "location");
    let result = result_intervals[0].start;
    println!("{result}");
}

fn main() {
    part_two();
}

use std::{collections::HashMap, io::stdin};

use phf::phf_map;

fn digits(l: &str) -> Vec<u32> {
    let mut r = Vec::new();
    for c in l.chars() {
        match c.to_digit(10) {
            Some(d) => r.push(d),
            None => {}
        }
    }

    return r;
}

fn part_one() {
    let mut total = 0u32;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let digits = digits(&line);
        total += digits[0] * 10 + digits[digits.len() - 1]
    }

    println!("{total}");
}

const DIGITS_MAP: phf::Map<&str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn first_and_last_digit(l: &str) -> Vec<u32> {
    let mut first = 0;
    let mut last = 0;

    let mut so_far = String::new();
    'out: for b in l.bytes() {
        let c = b as char;
        match c.to_digit(10) {
            Some(d) => {
                first = d;
                break;
            }
            None => {
                so_far.push(c);
                for (&k, &v) in DIGITS_MAP.entries() {
                    if so_far.ends_with(k) {
                        first = v;
                        break 'out;
                    }
                }
            }
        }
    }

    so_far = String::new();
    'out: for b in l.bytes().rev() {
        let c = b as char;
        match c.to_digit(10) {
            Some(d) => {
                last = d;
                break;
            }
            None => {
                so_far.insert(0, c);
                for (&k, &v) in DIGITS_MAP.entries() {
                    if so_far.starts_with(k) {
                        last = v;
                        break 'out;
                    }
                }
            }
        }
    }

    return vec![first, last];
}

fn part_two() {
    let mut total = 0u32;
    for line in stdin().lines().map(|l| l.unwrap()) {
        let digits = first_and_last_digit(&line);
        total += digits[0] * 10 + digits[digits.len() - 1]
    }

    println!("{total}");
}

fn main() {
    part_two();
}

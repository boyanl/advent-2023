use std::{collections::HashSet, io::stdin};

use scanf::sscanf;

struct Card {
    number: i32,
    winning: Vec<i32>,
    have: Vec<i32>,
}

fn read_input() -> Vec<Card> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut id: i32 = 0;
        let (mut winning_str, mut have_str) = (String::new(), String::new());
        if sscanf!(&line, "Card {i32}: {} | {}", id, winning_str, have_str).is_ok() {
            let winning = winning_str
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let have = have_str
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            result.push(Card {
                number: id,
                winning: winning,
                have: have,
            })
        }
    }

    return result;
}

fn count_winning(c: &Card) -> usize {
    return c.have.iter().filter(|&n| c.winning.contains(n)).count();
}

fn card_worth(c: &Card) -> i32 {
    let common_cnt = count_winning(c);

    if common_cnt == 0 {
        return 0;
    }
    return 1 << (common_cnt - 1);
}

fn part_one() {
    let cards = read_input();
    let result = cards.iter().map(|c| card_worth(c)).sum::<i32>();
    println!("{result}");
}

fn part_two() {
    let cards = read_input();

    let mut counts = vec![1; cards.len()];
    for i in 0..cards.len() {
        let cnt = count_winning(&cards[i]);

        for j in i + 1..=std::cmp::min(i + cnt, cards.len() - 1) {
            counts[j] += counts[i];
        }
    }

    let result = counts.iter().sum::<i32>();
    println!("{result}");
}

fn main() {
    part_two();
}

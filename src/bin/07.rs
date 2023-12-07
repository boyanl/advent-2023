use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    io::stdin,
};

#[derive(PartialEq, Clone, Copy, Debug)]
enum HandType {
    FiveKind(char),
    FourKind(char),
    FullHouse(char, char),
    ThreeKind(char),
    TwoPair(char, char),
    OnePair(char),
    HighCard(char),
}

fn strength(h: HandType) -> usize {
    match h {
        HandType::FiveKind(_) => 7,
        HandType::FourKind(_) => 6,
        HandType::FullHouse(_, _) => 5,
        HandType::ThreeKind(_) => 4,
        HandType::TwoPair(_, _) => 3,
        HandType::OnePair(_) => 2,
        HandType::HighCard(_) => 1,
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (strength1, strength2) = (strength(*self), strength(*other));

        if strength1 != strength2 {
            return Some(strength1.cmp(&strength2));
        }
        return Some(Ordering::Equal);
    }
}

const RANKS: &str = "23456789TJQKA";

fn rank_num(rank: char, ranks: &str) -> usize {
    return ranks.find(rank).unwrap();
}

fn hand_type(hand: &str) -> HandType {
    let mut counts = HashMap::new();
    for c in hand.chars() {
        let v = counts.entry(c).or_insert(0);
        *v += 1;
    }

    let mut counts_sorted = counts.iter().map(|(&c, &n)| (c, n)).collect::<Vec<_>>();
    counts_sorted.sort_by_key(|&(c, cnt)| cmp::Reverse(cnt));

    let highest_cnt = counts_sorted[0].1;
    let highest_rank = counts_sorted[0].0;

    match highest_cnt {
        5 => HandType::FiveKind(highest_rank),
        4 => HandType::FourKind(highest_rank),
        3 => {
            let (second_rank, second_highest_cnt) = counts_sorted[1];
            if second_highest_cnt == 2 {
                return HandType::FullHouse(highest_rank, second_rank);
            }
            return HandType::ThreeKind(highest_rank);
        }
        2 => {
            let (second_rank, second_highest_cnt) = counts_sorted[1];
            if second_highest_cnt == 2 {
                return HandType::TwoPair(highest_rank, second_rank);
            }
            return HandType::OnePair(highest_rank);
        }
        1 => HandType::HighCard(highest_rank),
        _ => todo!(),
    }
}

fn compare_hands(
    hand1: &str,
    hand2: &str,
    hand_type_fn: &Box<dyn Fn(&str) -> HandType>,
    rank_fn: &Box<dyn Fn(char) -> usize>,
) -> Ordering {
    let hand_type_cmp = hand_type_fn(hand1)
        .partial_cmp(&hand_type_fn(hand2))
        .unwrap();

    if hand_type_cmp != Ordering::Equal {
        return hand_type_cmp;
    }

    for (i, c) in hand1.chars().enumerate() {
        let c2 = hand2.chars().nth(i).unwrap();

        if c != c2 {
            return rank_fn(c).cmp(&rank_fn(c2));
        }
    }
    return Ordering::Equal;
}

fn rank_hands(
    hands: &Vec<(String, i32)>,
    hand_type_fn: Box<dyn Fn(&str) -> HandType>,
    rank_fn: Box<dyn Fn(char) -> usize>,
) -> Vec<(String, i32)> {
    let mut sorted = hands.clone();
    sorted.sort_by(|(h1, _), (h2, _)| compare_hands(h1, h2, &hand_type_fn, &rank_fn));

    return sorted;
}

fn read_input() -> Vec<(String, i32)> {
    let mut hands = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let split = line.split_ascii_whitespace().collect::<Vec<_>>();
        let (hand, bid) = (split[0], split[1].parse::<i32>().unwrap());
        hands.push((String::from(hand), bid));
    }
    hands
}

fn part_one() {
    let hands = read_input();

    let rank_fn = |c| rank_num(c, RANKS);
    let ranked = rank_hands(&hands, Box::new(hand_type), Box::new(rank_fn));
    let result = ranked
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| ((i + 1) as i64) * (bid as i64))
        .sum::<i64>();

    println!("{result}");
}

const RANKS_2: &str = "J23456789TQKA";

fn hand_type_2(hand: &str) -> HandType {
    let mut counts = HashMap::new();
    for c in hand.chars() {
        let v = counts.entry(c).or_insert(0);
        *v += 1;
    }

    let jokers_cnt = counts.get(&'J').map(|x| *x).unwrap_or(0);
    counts.remove(&'J');

    let mut counts_sorted = counts.iter().map(|(&c, &n)| (c, n)).collect::<Vec<_>>();
    counts_sorted.sort_by_key(|&(c, cnt)| cmp::Reverse(cnt));

    if jokers_cnt == 5 {
        return HandType::FiveKind('J');
    }

    let highest_cnt = counts_sorted[0].1;
    let highest_rank = counts_sorted[0].0;

    match highest_cnt + jokers_cnt {
        5 => HandType::FiveKind(highest_rank),
        4 => HandType::FourKind(highest_rank),
        3 => {
            let (second_rank, second_highest_cnt) = counts_sorted[1];
            if second_highest_cnt == 2 {
                return HandType::FullHouse(highest_rank, second_rank);
            }
            return HandType::ThreeKind(highest_rank);
        }
        2 => {
            let (second_rank, second_highest_cnt) = counts_sorted[1];
            if second_highest_cnt == 2 {
                return HandType::TwoPair(highest_rank, second_rank);
            }
            return HandType::OnePair(highest_rank);
        }
        1 => HandType::HighCard(highest_rank),
        _ => todo!(),
    }
}

fn part_two() {
    let hands = read_input();

    let rank_fn = |c| rank_num(c, RANKS_2);
    let ranked = rank_hands(&hands, Box::new(hand_type_2), Box::new(rank_fn));

    let result = ranked
        .iter()
        .enumerate()
        .map(|(i, &(_, bid))| ((i + 1) as i64) * (bid as i64))
        .sum::<i64>();

    println!("{result}");
}

fn main() {
    part_two();
}

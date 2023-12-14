use std::{collections::HashMap, io::stdin};

type Map = Vec<Vec<char>>;

fn print_input(m: &Map) {
    for line in m.iter() {
        let s: String = line.iter().collect();
        println!("{s}");
    }
}

fn read_input() -> Map {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        result.push(line.chars().collect());
    }

    result
}

fn tilt_north(map: &Map) -> Map {
    let mut res = map.clone();

    let (n, m) = (res.len(), res[0].len());

    for j in 0..m {
        for i in 0..n {
            if res[i][j] == 'O' {
                let mut k = i as i32 - 1;

                while k >= 0 && k < (n as i32) && res[k as usize][j] == '.' {
                    k -= 1;
                }
                if k + 1 != i as i32 {
                    res[(k + 1) as usize][j] = 'O';
                    res[i][j] = '.';
                }
            }
        }
    }

    res
}

// Much copy-paste 🐕
fn tilt_south(map: &Map) -> Map {
    let mut res = map.clone();

    let (n, m) = (res.len(), res[0].len());

    for j in 0..m {
        for i in (0..n).rev() {
            if res[i][j] == 'O' {
                let mut k = i as i32 + 1;

                while k >= 0 && k < (n as i32) && res[k as usize][j] == '.' {
                    k += 1;
                }
                if k - 1 != i as i32 {
                    res[(k - 1) as usize][j] = 'O';
                    res[i][j] = '.';
                }
            }
        }
    }

    res
}

fn tilt_west(map: &Map) -> Map {
    let mut res = map.clone();

    let (n, m) = (res.len(), res[0].len());

    for i in 0..n {
        for j in 0..m {
            if res[i][j] == 'O' {
                let mut k = j as i32 - 1;

                while k >= 0 && k < (m as i32) && res[i][k as usize] == '.' {
                    k -= 1;
                }
                if k + 1 != j as i32 {
                    res[i][(k + 1) as usize] = 'O';
                    res[i][j] = '.';
                }
            }
        }
    }

    res
}

fn tilt_east(map: &Map) -> Map {
    let mut res = map.clone();

    let (n, m) = (res.len(), res[0].len());

    for i in 0..n {
        for j in (0..m).rev() {
            if res[i][j] == 'O' {
                let mut k = j as i32 + 1;

                while k >= 0 && k < (m as i32) && res[i][k as usize] == '.' {
                    k += 1;
                }
                if k - 1 != j as i32 {
                    res[i][(k - 1) as usize] = 'O';
                    res[i][j] = '.';
                }
            }
        }
    }

    res
}

fn simulate(map: &Map, rounds: i32) -> Map {
    let mut round = 1;
    let mut current = map.clone();
    let mut seen: HashMap<Map, i32> = HashMap::new();

    while round <= rounds {
        current = tilt_east(&tilt_south(&tilt_west(&tilt_north(&current))));

        if let Some(&rnd) = seen.get(&current) {
            let cycle_len = round - rnd;
            let remaining = rounds - round;
            let cycles = remaining / cycle_len;

            round += cycles * cycle_len;
        } else {
            seen.insert(current.clone(), round);
        }
        round += 1;
    }

    current
}

fn score(map: &Map) -> i32 {
    let n = map.len();
    let mut res = 0;

    for (i, line) in map.iter().enumerate() {
        for c in line.iter() {
            if *c == 'O' {
                res += n - i;
            }
        }
    }

    res as i32
}

fn part_one() {
    let input = read_input();
    let result = score(&tilt_north(&input));

    println!("{result}");
}

fn part_two() {
    let input = read_input();
    let rounds = 1_000_000_000;

    let final_map = simulate(&input, rounds);
    // let final_map = tilt_north(&input);
    println!("{}", score(&final_map));
}

fn main() {
    part_two();
}

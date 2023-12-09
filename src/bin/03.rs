use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};

fn neighbours(pos: (usize, usize), limits: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            if di != 0 || dj != 0 {
                let (ni, nj) = (pos.0 as i32 + di, pos.1 as i32 + dj);
                if ni >= 0 && ni < limits.0 as i32 && nj >= 0 && nj < limits.1 as i32 {
                    result.push((ni as usize, nj as usize));
                }
            }
        }
    }

    result
}

fn char(s: &str, i: usize) -> char {
    s.as_bytes()[i] as char
}

fn part_one() {
    let mut result = 0;
    let lines = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;
        let n = line.chars().count();
        while j < n {
            if char(line, j).is_digit(10) {
                let mut have_symbol = false;
                let mut val = 0;

                while j < n {
                    let c = char(line, j);
                    if !c.is_digit(10) {
                        break;
                    }

                    have_symbol |= neighbours((i, j), (lines.len(), n))
                        .iter()
                        .any(|&(i1, j1)| {
                            let c = lines[i1 as usize].chars().nth(j1 as usize).unwrap();
                            return !c.is_digit(10) && c != '.';
                        });
                    val = val * 10 + (c as i32) - ('0' as i32);
                    j += 1;
                }

                if have_symbol {
                    result += val;
                }
            }
            j += 1;
        }
    }
    println!("{result}");
}

fn number_including(i: usize, j: usize, lines: &Vec<String>) -> (i32, (usize, usize)) {
    let (mut start, mut end) = (j as i32, j);
    let line = &lines[i];

    while start >= 0 && char(line, start as usize).is_ascii_digit() {
        start -= 1;
    }

    while end < line.len() && char(line, end).is_ascii_digit() {
        end += 1;
    }

    let start = (start + 1) as usize;
    return (line[start..end].parse::<i32>().unwrap(), (i, start));
}

fn adjacent_numbers(i: usize, j: usize, lines: &Vec<String>) -> Vec<i32> {
    let mut number_pos: HashMap<(usize, usize), i32> = HashMap::new();
    let limits = (lines.len(), lines[0].len());

    for cell in neighbours((i, j), limits) {
        let c = char(&lines[cell.0 as usize], cell.1 as usize);
        if c.is_ascii_digit() {
            let (val, pos) = number_including(cell.0, cell.1, lines);
            number_pos.insert(pos, val);
        }
    }

    number_pos.values().map(|&v| v).collect::<Vec<_>>()
}

fn part_two() {
    let mut result = 0i64;
    let lines = stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                let numbers = adjacent_numbers(i, j, &lines);
                if numbers.len() == 2 {
                    result += (numbers[0] * numbers[1]) as i64;
                }
            }
        }
    }
    println!("{result}");
}

fn main() {
    part_two();
}

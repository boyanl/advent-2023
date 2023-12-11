use std::{cmp, collections::HashSet, io::stdin};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

type Input = (Vec<Vec<char>>, HashSet<usize>, HashSet<usize>);

fn read_input() -> Input {
    let mut lines = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        lines.push(line.chars().collect::<Vec<_>>());
    }

    let mut empty_rows = HashSet::new();
    for (i, line) in lines.iter().enumerate() {
        if line.iter().position(|c| *c == '#').is_none() {
            empty_rows.insert(i);
        }
    }

    let mut empty_cols = HashSet::new();
    for j in 0..lines[0].len() {
        let mut have_galaxy = false;
        for i in 0..lines.len() {
            if lines[i][j] == '#' {
                have_galaxy = true;
                break;
            }
        }

        if !have_galaxy {
            empty_cols.insert(j);
        }
    }

    return (lines, empty_rows, empty_cols);
}

fn galaxy_positions(map: &Vec<Vec<char>>) -> Vec<Pos> {
    let mut galaxies = Vec::new();
    for (i, line) in map.iter().enumerate() {
        let row_galaxies = line
            .iter()
            .enumerate()
            .filter(|&(_, c)| *c == '#')
            .map(|(j, c)| j)
            .collect::<Vec<_>>();

        for j in row_galaxies {
            galaxies.push(Pos {
                x: j as i32,
                y: i as i32,
            });
        }
    }

    galaxies
}

fn dist(
    g1: Pos,
    g2: Pos,
    empty_rows: &HashSet<usize>,
    empty_cols: &HashSet<usize>,
    empty_size: i32,
) -> i64 {
    let mut d = ((g1.x - g2.x).abs() + (g1.y - g2.y).abs()) as i64;

    let (min_y, max_y) = (cmp::min(g1.y, g2.y), cmp::max(g1.y, g2.y));
    for r in empty_rows {
        let r = *r as i32;
        if min_y < r && r < max_y {
            d += (empty_size - 1) as i64;
        }
    }

    let (min_x, max_x) = (cmp::min(g1.x, g2.x), cmp::max(g1.x, g2.x));
    for c in empty_cols {
        let c = *c as i32;
        if min_x < c && c < max_x {
            d += (empty_size - 1) as i64;
        }
    }

    d
}

fn part_one() {
    let (map, empty_rows, empty_cols) = read_input();
    let galaxies = galaxy_positions(&map);

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let d = dist(galaxies[i], galaxies[j], &empty_rows, &empty_cols, 2);
            result += d;
        }
    }

    println!("{result}")
}

fn part_two() {
    let (map, empty_rows, empty_cols) = read_input();
    let galaxies = galaxy_positions(&map);

    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let d = dist(
                galaxies[i],
                galaxies[j],
                &empty_rows,
                &empty_cols,
                1_000_000,
            );
            result += d;
        }
    }

    println!("{result}")
}

fn main() {
    part_two();
}

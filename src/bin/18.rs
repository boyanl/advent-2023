use std::{
    io::stdin,
    ops::{Add, Mul, Sub},
};

use scanf::sscanf;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

type Dir = Pos;
const UP: Dir = Dir { x: 0, y: -1 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };

impl Add<Dir> for Pos {
    type Output = Pos;
    fn add(self, rhs: Dir) -> Self::Output {
        return Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub<Pos> for Pos {
    type Output = Dir;
    fn sub(self, rhs: Pos) -> Self::Output {
        return Dir {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Mul<i32> for Dir {
    type Output = Dir;
    fn mul(self, rhs: i32) -> Self::Output {
        return Dir {
            x: rhs * self.x,
            y: rhs * self.y,
        };
    }
}

#[derive(Debug)]
struct Dig {
    dir: Dir,
    amount: i32,
}

fn read_input() -> Vec<Dig> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut dir_str, mut amount_str, mut color_str) =
            (String::new(), String::new(), String::new());
        if sscanf!(&line, "{} {} (#{})", dir_str, amount_str, color_str).is_ok() {
            let dir = match dir_str.as_str() {
                "R" => RIGHT,
                "L" => LEFT,
                "D" => DOWN,
                "U" => UP,
                _ => todo!(),
            };
            let amount = amount_str.parse::<i32>().unwrap();
            result.push(Dig {
                dir: dir,
                amount: amount,
            });
        }
    }
    result
}

fn area(polygon: &Vec<Pos>) -> i64 {
    let mut total = 0i64;

    for i in 0..polygon.len() {
        let (p1, p2) = (polygon[i], polygon[(i + 1) % polygon.len()]);
        total += ((p2.x - p1.x) as i64) * ((p2.y + p1.y) as i64)
    }

    total.abs() / 2i64
}

fn total_area(input: &Vec<Dig>) -> i64 {
    let mut coords = Vec::new();
    let mut current = Pos { x: 0, y: 0 };
    coords.push(current);

    let mut perimeter = 0;

    for dig in input {
        let new_current = current + dig.dir * dig.amount;

        perimeter += dig.amount;

        current = new_current;
        coords.push(current);
    }

    assert!(current == Pos { x: 0, y: 0 });
    let area_small = area(&coords);
    let area_all = area_small + (perimeter / 2 + 1) as i64;

    area_all
}

fn part_one() {
    let input = read_input();
    let result = total_area(&input);

    println!("{result}");
}

fn read_input_2() -> Vec<Dig> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut dir_str, mut amount_str, mut color_str) =
            (String::new(), String::new(), String::new());
        if sscanf!(&line, "{} {} (#{})", dir_str, amount_str, color_str).is_ok() {
            let amount = i32::from_str_radix(&color_str[0..5], 16).unwrap();
            let dir = match color_str.chars().nth(5).unwrap() {
                '0' => RIGHT,
                '1' => DOWN,
                '2' => LEFT,
                '3' => UP,
                _ => todo!(),
            };
            result.push(Dig {
                dir: dir,
                amount: amount,
            });
        }
    }
    result
}

fn part_two() {
    let input = read_input_2();
    let result = total_area(&input);

    println!("{result}");
}

fn main() {
    part_two();
}

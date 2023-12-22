use cmp::{max, min};
use std::{cmp, collections::HashSet, io::stdin};

use scanf::sscanf;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Box {
    min: Point3,
    max: Point3,
}

type Brick = Box;

fn intersection(b1: Box, b2: Box) -> Box {
    let min_pt = Point3 {
        x: max(b1.min.x, b2.min.x),
        y: max(b1.min.y, b2.min.y),
        z: max(b1.min.z, b2.min.z),
    };
    let max_pt = Point3 {
        x: min(b1.max.x, b2.max.x),
        y: min(b1.max.y, b2.max.y),
        z: min(b1.max.z, b2.max.z),
    };

    return Box {
        min: min_pt,
        max: max_pt,
    };
}

fn is_empty(b: Box) -> bool {
    b.min.x >= b.max.x || b.min.y >= b.max.y || b.min.z >= b.max.z
}

fn intersect(b1: Brick, b2: Brick) -> bool {
    !is_empty(intersection(b1, b2))
}

fn read_input() -> Vec<(usize, Brick)> {
    let mut result = Vec::new();
    for (i, line) in stdin().lines().map(|l| l.unwrap()).enumerate() {
        let (mut sx, mut sy, mut sz, mut ex, mut ey, mut ez) = (0, 0, 0, 0, 0, 0);
        if sscanf!(
            &line,
            "{i32},{i32},{i32}~{i32},{i32},{i32}",
            sx,
            sy,
            sz,
            ex,
            ey,
            ez
        )
        .is_ok()
        {
            if sx > ex || sy > ey || sz > ez {
                println!("Ey");
            }
            let brick = Brick {
                min: Point3 {
                    x: sx,
                    y: sy,
                    z: sz,
                },
                max: Point3 {
                    x: ex + 1,
                    y: ey + 1,
                    z: ez + 1,
                },
            };
            result.push((i, brick));
        }
    }
    result
}

fn fall(bricks: &Vec<(usize, Brick)>) -> Vec<(usize, Brick)> {
    let mut result = bricks.clone();

    result.sort_by_key(|&(_, b)| b.min.z);

    let mut fallen: HashSet<Box> = HashSet::new();

    for (_, brick) in result.iter_mut() {
        'out: loop {
            if brick.min.z == 1 {
                fallen.insert(*brick);
                break;
            }
            let mut one_down = brick.clone();
            one_down.min.z -= 1;
            one_down.max.z -= 1;

            for &other in &fallen {
                if intersect(one_down, other) {
                    fallen.insert(*brick);
                    break 'out;
                }
            }

            *brick = one_down;
        }
    }

    result.sort_by_key(|(i, _)| *i);

    result
}

fn part_one() {
    let bricks = read_input();
    let stable_config = fall(&bricks);

    assert!(fall(&stable_config) == stable_config);

    let mut result = 0;
    for i in 0..stable_config.len() {
        let mut with_removed = stable_config.clone();
        with_removed.remove(i);

        if fall(&with_removed) == with_removed {
            result += 1;
        }
    }

    println!("{result}");
}

fn part_two() {
    let bricks = read_input();
    let stable_config = fall(&bricks);

    assert!(fall(&stable_config) == stable_config);
    let stable_config_set: HashSet<(usize, Box)> = HashSet::from_iter(stable_config.clone());

    let mut result = 0;
    for i in 0..stable_config.len() {
        let mut with_removed = stable_config.clone();
        with_removed.remove(i);

        let fell_set: HashSet<(usize, Box)> = HashSet::from_iter(fall(&with_removed));
        result += fell_set.difference(&stable_config_set).count();
    }

    println!("{result}");
}

fn main() {
    part_two();
}

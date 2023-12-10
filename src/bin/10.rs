use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

type Dir = Pos;
const LEFT: Dir = Pos { x: -1, y: 0 };
const RIGHT: Dir = Pos { x: 1, y: 0 };
const UP: Dir = Pos { x: 0, y: -1 };
const DOWN: Dir = Pos { x: 0, y: 1 };

type Map = Vec<Vec<char>>;

fn animal_position(map: &Map) -> Pos {
    for i in 0..map.len() {
        if let Some(j) = map[i].iter().position(|c| *c == 'S') {
            return Pos {
                x: j as i32,
                y: i as i32,
            };
        }
    }
    todo!();
}

fn neighbours(pos: Pos, map: &Map) -> Vec<Pos> {
    let c = map[pos.y as usize][pos.x as usize];
    let mut res = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            if (dx == 0) ^ (dy == 0) {
                let (nx, ny) = (pos.x + dx, pos.y + dy);

                if nx >= 0 && nx < map[0].len() as i32 && ny >= 0 && ny < map.len() as i32 {
                    let c_next = map[ny as usize][nx as usize];

                    let from_left = dx == 1 && "J7-".contains(c_next);
                    let from_right = dx == -1 && "LF-".contains(c_next);
                    let from_top = dy == 1 && "JL|".contains(c_next);
                    let from_bottom = dy == -1 && "F7|".contains(c_next);

                    let ok = match c {
                        'S' => from_left || from_right || from_top || from_bottom,
                        '-' => from_left || from_right,
                        '|' => from_top || from_bottom,
                        'L' => from_bottom || from_left,
                        '7' => from_top || from_right,
                        'F' => from_top || from_left,
                        'J' => from_bottom || from_right,
                        '.' => false,
                        _ => todo!(),
                    };

                    if ok {
                        res.push(Pos { x: nx, y: ny });
                    }
                }
            }
        }
    }

    res
}

fn farthest_from(pos: Pos, map: &Map) -> (Pos, i32) {
    let mut q = VecDeque::new();
    q.push_back((pos, 0));

    let mut visited = HashSet::new();
    visited.insert(pos);

    let mut max_dist = -1;
    let mut max_dist_pos = Pos { x: 0, y: 0 };

    while !q.is_empty() {
        let (curr, d) = q.pop_front().unwrap();
        if d > max_dist {
            max_dist = d;
            max_dist_pos = curr;
        }

        for n in neighbours(curr, map) {
            if !visited.contains(&n) {
                q.push_back((n, d + 1));
                visited.insert(n);
            }
        }
    }

    (max_dist_pos, max_dist)
}

fn part_one() {
    let mut map = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        map.push(line.chars().collect::<Vec<_>>());
    }

    let animal_pos = animal_position(&map);
    let (_, dist) = farthest_from(animal_pos, &map);

    println!("{dist}")
}

fn loop_points(start: Pos, map: &Map) -> HashSet<Pos> {
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    while !q.is_empty() {
        let (curr, d) = q.pop_front().unwrap();

        for n in neighbours(curr, map) {
            if !visited.contains(&n) {
                q.push_back((n, d + 1));
                visited.insert(n);
            }
        }
    }

    visited
}

fn direction(p1: Pos, p2: Pos) -> Dir {
    let (dx, dy) = (p2.x - p1.x, p2.y - p1.y);
    assert!(dx == 0 || dy == 0);
    Pos {
        x: if dx == 0 { 0 } else { dx / dx.abs() },
        y: if dy == 0 { 0 } else { dy / dy.abs() },
    }
}

fn advance(p: Pos, d: Dir) -> Pos {
    return Pos {
        x: p.x + d.x,
        y: p.y + d.y,
    };
}

fn right(p: Pos, dir: Dir) -> Pos {
    match dir {
        LEFT => advance(p, UP),
        RIGHT => advance(p, DOWN),
        UP => advance(p, RIGHT),
        DOWN => advance(p, LEFT),
        _ => todo!(),
    }
}

fn left(p: Pos, dir: Dir) -> Pos {
    match dir {
        LEFT => advance(p, DOWN),
        RIGHT => advance(p, UP),
        UP => advance(p, LEFT),
        DOWN => advance(p, RIGHT),
        _ => todo!(),
    }
}

fn generate_path(start: Pos, loop_pts: &HashSet<Pos>, map: &Map) -> Vec<Pos> {
    let mut path = Vec::new();
    let mut current = start;
    let mut visited = HashSet::new();

    while path.len() < loop_pts.len() {
        let next = neighbours(current, &map)
            .iter()
            .filter(|&p| !visited.contains(p))
            .map(|&p| p)
            .collect::<Vec<_>>();

        if next.len() > 1 {
            assert!(current == start);
        }

        path.push(current);
        visited.insert(current);

        if !next.is_empty() {
            current = next[0];
        } else {
            assert!(path.len() == loop_pts.len());
        }
    }

    path
}

fn in_bounds(p: Pos, map: &Map) -> bool {
    return p.x >= 0 && p.x < map[0].len() as i32 && p.y >= 0 && p.y < map.len() as i32;
}

fn flood_fill(
    pos: Pos,
    loop_pts: &HashSet<Pos>,
    map: &Map,
    visited: &mut HashSet<Pos>,
) -> (i32, bool) {
    let mut result = 0;
    let mut q = VecDeque::new();
    q.push_back(pos);

    visited.insert(pos);

    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        result += 1;

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let new_p = advance(p, dir);
            if !in_bounds(new_p, map) {
                return (result, true);
            }

            if !loop_pts.contains(&new_p) && !visited.contains(&new_p) {
                visited.insert(new_p);
                q.push_back(new_p);
            }
        }
    }

    return (result, false);
}

fn count_inside(
    path: &Vec<Pos>,
    clockwise: bool,
    path_pts: &HashSet<Pos>,
    map: &Map,
) -> (HashSet<Pos>, bool) {
    let mut visited = HashSet::new();

    for i in 0..path.len() {
        let from = path[i];
        let to = path[(i + 1) % path.len()];
        let dir = direction(from, to);
        let (p1, p2) = if clockwise {
            (right(from, dir), right(to, dir))
        } else {
            (left(from, dir), left(to, dir))
        };

        for pos in [p1, p2] {
            if !in_bounds(pos, map) {
                return (visited, true);
            }

            if !path_pts.contains(&pos) && !visited.contains(&pos) {
                let (cnt, out) = flood_fill(pos, path_pts, map, &mut visited);
                if out {
                    return (visited, true);
                }
            }
        }
    }

    (visited, false)
}

fn count_inside_2(loop_pts: &HashSet<Pos>, map: &Map) -> HashSet<Pos> {
    let mut res = HashSet::new();
    for (i, line) in map.iter().enumerate() {
        let mut inside = false;
        let mut segment_start = 'a';
        for (j, &c) in line.iter().enumerate() {
            let pos = Pos {
                x: j as i32,
                y: i as i32,
            };
            let in_loop = loop_pts.contains(&pos);

            if !in_loop && inside {
                res.insert(pos);
            } else if in_loop {
                if c == '|' {
                    inside = !inside;
                } else if "FL".contains(c) {
                    segment_start = c;
                } else if "FL7J".contains(c) {
                    if segment_start == 'F' && c == 'J' {
                        inside = !inside;
                    } else if segment_start == 'L' && c == '7' {
                        inside = !inside;
                    }
                }
            }
        }
    }

    res
}

fn replace_starting_pos(path: &Vec<Pos>, map: &mut Map) {
    let start = path[0];
    let (d1, d2) = (
        direction(path[path.len() - 1], path[0]),
        direction(path[0], path[1]),
    );

    let c = match d1 {
        UP => match d2 {
            RIGHT => 'F',
            UP => '|',
            LEFT => '7',
            _ => todo!(),
        },
        DOWN => match d2 {
            RIGHT => 'L',
            DOWN => '|',
            LEFT => 'J',
            _ => todo!(),
        },
        LEFT => match d2 {
            UP => 'L',
            DOWN => 'F',
            LEFT => '-',
            _ => todo!(),
        },
        RIGHT => match d2 {
            UP => 'J',
            DOWN => '7',
            RIGHT => '-',
            _ => todo!(),
        },
        _ => todo!(),
    };

    map[start.y as usize][start.x as usize] = c;
}

fn part_two() {
    let mut map = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        map.push(line.chars().collect::<Vec<_>>());
    }

    let animal_pos = animal_position(&map);
    let loop_pts = loop_points(animal_pos, &map);

    let path = generate_path(animal_pos, &loop_pts, &map);
    replace_starting_pos(&path, &mut map);

    for clockwise in [true, false] {
        let (positions, outside) = count_inside(&path, clockwise, &loop_pts, &map);
        if !outside {
            println!("{}", positions.len());
            break;
        }
    }
}

fn main() {
    part_two();
}

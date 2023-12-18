use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    io::stdin,
    ops::Add,
};

fn read_input() -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let mut line_costs = Vec::new();
        for c in line.chars() {
            line_costs.push(c.to_digit(10).unwrap() as i32);
        }
        result.push(line_costs)
    }

    result
}

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

const DIRS_CLOCKWISE: [Dir; 4] = [UP, RIGHT, DOWN, LEFT];

fn rotate_right(d: Dir) -> Dir {
    let idx = DIRS_CLOCKWISE.iter().position(|&x| x == d).unwrap();
    return DIRS_CLOCKWISE[(idx + 1) % 4];
}

fn rotate_left(d: Dir) -> Dir {
    let idx = DIRS_CLOCKWISE.iter().position(|&x| x == d).unwrap();
    return DIRS_CLOCKWISE[(idx + 3) % 4];
}

fn opposite(d: Dir) -> Dir {
    Dir { x: -d.x, y: -d.y }
}

type State = (i32, Pos, Dir, i32);

fn in_bounds(p: Pos, map: &Vec<Vec<i32>>) -> bool {
    return (p.y >= 0 && p.y < map.len() as i32) && (p.x >= 0 && p.x < map[0].len() as i32);
}

fn next_states_1(state: State, map: &Vec<Vec<i32>>) -> Vec<State> {
    let forbidden_dirs;
    if state.3 == 3 {
        forbidden_dirs = vec![state.2, opposite(state.2)];
    } else {
        forbidden_dirs = vec![opposite(state.2)];
    }
    let dirs = [UP, DOWN, LEFT, RIGHT]
        .iter()
        .filter(|&d| !forbidden_dirs.contains(d))
        .map(|d| *d)
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    for dir in dirs {
        let new_pos = state.1 + dir;
        if in_bounds(new_pos, map) {
            let cost = map[new_pos.y as usize][new_pos.x as usize];
            let consecutive = if dir == state.2 { state.3 + 1 } else { 1 };
            result.push((state.0 + cost, new_pos, dir, consecutive));
        }
    }
    result
}

fn is_final_1(state: State, map: &Vec<Vec<i32>>) -> bool {
    let pos = state.1;
    let (n, m) = (map.len() as i32, map[0].len() as i32);
    return (pos.y == n - 1) && (pos.x == m - 1);
}

fn find_path(
    map: &Vec<Vec<i32>>,
    state_fn: fn(State, &Vec<Vec<i32>>) -> Vec<State>,
    is_final: fn(State, &Vec<Vec<i32>>) -> bool,
) -> i32 {
    let mut q = BinaryHeap::new();
    let start = Pos { x: 0, y: 0 };

    let start_state = (0, start, DOWN, 0);
    q.push(Reverse(start_state));

    let mut seen = HashSet::new();

    while !q.is_empty() {
        let state = q.pop().unwrap().0;
        if is_final(state, map) {
            return state.0;
        }

        let seen_key = (state.1, state.2, state.3);
        if seen.contains(&seen_key) {
            continue;
        }

        seen.insert(seen_key);

        for next_state in state_fn(state, map) {
            q.push(Reverse(next_state));
        }
    }
    -1
}

fn part_one() {
    let input = read_input();
    let result = find_path(&input, next_states_1, is_final_1);
    println!("{result}");
}

fn next_states_2(state: State, map: &Vec<Vec<i32>>) -> Vec<State> {
    let dirs;
    if state.3 >= 1 && state.3 < 4 {
        dirs = vec![state.2];
    } else if state.3 >= 10 {
        dirs = vec![rotate_left(state.2), rotate_right(state.2)];
    } else {
        dirs = vec![state.2, rotate_left(state.2), rotate_right(state.2)];
    }

    let mut result = Vec::new();
    for dir in dirs {
        let new_pos = state.1 + dir;
        if in_bounds(new_pos, map) {
            let cost = map[new_pos.y as usize][new_pos.x as usize];
            let consecutive = if dir == state.2 { state.3 + 1 } else { 1 };
            result.push((state.0 + cost, new_pos, dir, consecutive));
        }
    }
    result
}

fn is_final_2(state: State, map: &Vec<Vec<i32>>) -> bool {
    let pos = state.1;
    let (n, m) = (map.len() as i32, map[0].len() as i32);
    return (pos.y == n - 1) && (pos.x == m - 1) && state.3 >= 4;
}

fn part_two() {
    let input = read_input();
    let result = find_path(&input, next_states_2, is_final_2);
    println!("{result}");
}

fn main() {
    part_two();
}

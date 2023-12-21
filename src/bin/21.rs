use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

fn read_input() -> (Vec<Vec<char>>, Pos) {
    let mut map = Vec::new();
    let mut start: Pos = Pos { x: 0, y: 0 };
    for (i, line) in stdin().lines().map(|l| l.unwrap()).enumerate() {
        map.push(line.chars().collect());

        if let Some(j) = line.find("S") {
            start = Pos { x: j, y: i };
        }
    }

    (map, start)
}

fn neighbours(p: Pos, map: &Vec<Vec<char>>) -> Vec<Pos> {
    let (n, m) = (map.len(), map[0].len());
    let mut result = Vec::new();

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nx, ny) = ((p.x as i32 + dx) as usize, (p.y as i32 + dy) as usize);
        if nx < m && ny < n && map[ny][nx] == '.' {
            result.push(Pos { x: nx, y: ny });
        }
    }

    result
}

fn show_reachable(ps: &HashSet<Pos>, map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                print!("S");
            } else if ps.contains(&Pos { x: j, y: i }) {
                print!("O");
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }
}

fn reachable_count(start: Pos, d: i32, map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut res = 0;
    let mut max_dist = -1;

    while !q.is_empty() {
        let (pos, dist) = q.pop_front().unwrap();
        if dist % 2 == d % 2 {
            res += 1;
            if dist > max_dist {
                max_dist = dist;
            }
        }

        for n in neighbours(pos, map) {
            if !visited.contains(&n) && dist + 1 <= d {
                visited.insert(n);
                q.push_back((n, dist + 1));
            }
        }
    }

    (res, max_dist)
}

fn part_one() {
    let (map, start) = read_input();
    let steps = 64;
    let (result, _) = reachable_count(start, steps, &map);

    println!("{result}");
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Pos2 {
    x: i32,
    y: i32,
}

fn neighbours_2(p: Pos2, map: &Vec<Vec<char>>) -> Vec<Pos2> {
    let (n, m) = (map.len(), map[0].len());
    let mut result = Vec::new();

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nx, ny) = ((p.x as i32 + dx), (p.y as i32 + dy));
        let (mx, my) = (nx.rem_euclid(m as i32), ny.rem_euclid(n as i32));
        let c = map[my as usize][mx as usize];
        if c == '.' || c == 'S' {
            result.push(Pos2 { x: nx, y: ny });
        }
    }

    result
}

fn reachable_count_2_naive(start: Pos2, d: i32, map: &Vec<Vec<char>>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut res = 0;
    let mut cells = HashSet::new();

    while !q.is_empty() {
        let (pos, dist) = q.pop_front().unwrap();
        if dist % 2 == d % 2 {
            res += 1;
            cells.insert(pos);
        }

        for n in neighbours_2(pos, map) {
            if !visited.contains(&n) && dist + 1 <= d {
                visited.insert(n);
                q.push_back((n, dist + 1));
            }
        }
    }

    res
}

fn reachable_count_2_smart(start: Pos2, max_dist: i32, map: &Vec<Vec<char>>) -> i64 {
    let mut q = HashSet::new();
    q.insert(start);

    let len = map.len() as i32;
    let half_len = len / 2;

    assert!((max_dist - half_len) % len == 0);

    let mut last_count = 0i64;
    let mut last_increase = 0i64;
    let mut last_increase_diff = 0i64;
    let mut count = 0i64;

    let mut d = 0;
    while d <= max_dist {
        let mut new_q = HashSet::new();
        count = q.len() as i64;
        for pos in q {
            for n in neighbours_2(pos, map) {
                if d + 1 <= max_dist {
                    new_q.insert(n);
                }
            }
        }

        // We know that the total count is a function of n^2, e.g. f(x) = a*x^2 + b*x + c
        // In discrete terms, increase ~= f'(x), increase_diff ~= f''(x), so since increase_diff_2 = f'''(x), it must become 0 from some point on
        // There is some noise at the start, but from a certain point on this seems to be true (even in the discrete case)
        // Note - since the pattern allows traveling freely straight up/down/left/right from the center
        // and since max_dist = len/2 + k*len,
        // so we only keep track of the answer at such distances
        if d % len == half_len {
            let increase = count - last_count;
            let increase_diff = increase - last_increase;
            let increase_diff_2 = increase_diff - last_increase_diff;

            // println!("Count = {count}, last_count = {last_count}, increase = {increase}, increase_diff = {increase_diff}, increase_diff_2 = {increase_diff_2}");

            last_count = count;
            last_increase = increase;
            last_increase_diff = increase_diff;
            if increase_diff_2 == 0 {
                break;
            }
        }

        q = new_q;
        d += 1;
    }

    while d < max_dist {
        last_increase += last_increase_diff;
        count += last_increase;
        d += len;
    }

    count
}

fn part_two() {
    let (map, start) = read_input();

    let start = Pos2 {
        x: start.x as i32,
        y: start.y as i32,
    };
    let steps = 26501365;
    let result = reachable_count_2_smart(start, steps, &map);
    println!("{result}");
}

fn main() {
    part_two();
}

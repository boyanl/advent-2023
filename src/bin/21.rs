use std::{
    collections::{HashSet, VecDeque},
    io::stdin,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

fn read_input() -> (Vec<Vec<char>>, Pos) {
    let mut map = Vec::new();
    let mut start: Pos = Pos { x: 0, y: 0 };
    for (i, line) in stdin().lines().map(|l| l.unwrap()).enumerate() {
        map.push(line.chars().collect());

        if let Some(j) = line.find("S") {
            start = Pos {
                x: j as i32,
                y: i as i32,
            };
        }
    }

    (map, start)
}

fn neighbours(p: Pos, map: &Vec<Vec<char>>) -> Vec<Pos> {
    let (n, m) = (map.len(), map[0].len());
    let mut result = Vec::new();

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nx, ny) = (p.x + dx, p.y + dy);
        if nx >= 0
            && nx < m as i32
            && ny >= 0
            && ny < n as i32
            && "S.".contains(map[ny as usize][nx as usize])
        {
            result.push(Pos { x: nx, y: ny });
        }
    }

    result
}

fn reachable_count(start: Pos, d: i32, map: &Vec<Vec<char>>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut res = 0;

    while !q.is_empty() {
        let (pos, dist) = q.pop_front().unwrap();
        if dist % 2 == d % 2 {
            res += 1;
        }

        for n in neighbours(pos, map) {
            if !visited.contains(&n) && dist + 1 <= d {
                visited.insert(n);
                q.push_back((n, dist + 1));
            }
        }
    }

    res
}

fn part_one() {
    let (map, start) = read_input();
    let steps = 64;
    let result = reachable_count(start, steps, &map);

    println!("{result}");
}

fn neighbours_2(p: Pos, map: &Vec<Vec<char>>) -> Vec<Pos> {
    let (n, m) = (map.len(), map[0].len());
    let mut result = Vec::new();

    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nx, ny) = ((p.x as i32 + dx), (p.y as i32 + dy));
        let (mx, my) = (nx.rem_euclid(m as i32), ny.rem_euclid(n as i32));
        let c = map[my as usize][mx as usize];
        if c == '.' || c == 'S' {
            result.push(Pos { x: nx, y: ny });
        }
    }

    result
}

fn reachable_cells_naive(start: Pos, d: i32, map: &Vec<Vec<char>>) -> HashSet<Pos> {
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut cells = HashSet::new();

    while !q.is_empty() {
        let (pos, dist) = q.pop_front().unwrap();
        if dist % 2 == d % 2 {
            cells.insert(pos);
        }

        for n in neighbours_2(pos, map) {
            if !visited.contains(&n) && dist + 1 <= d {
                visited.insert(n);
                q.push_back((n, dist + 1));
            }
        }
    }

    cells
}

fn visualize_reachable_cells(cells: &HashSet<Pos>, map: &Vec<Vec<char>>) {
    let (h, w) = (map.len() as i32, map[0].len() as i32);

    let min_x = cells.iter().map(|p| p.x).min().unwrap();
    let min_y = cells.iter().map(|p| p.y).min().unwrap();
    let max_x = cells.iter().map(|p| p.x).max().unwrap();
    let max_y = cells.iter().map(|p| p.y).max().unwrap();

    for y in min_y - 1..=max_y + 1 {
        if y % h == 0 {
            println!();
        }
        for x in min_x - 1..=max_x + 1 {
            if x % w == 0 {
                print!(" ");
            }
            let pos = Pos { x, y };
            let (mx, my) = (x.rem_euclid(w) as usize, y.rem_euclid(h) as usize);
            if map[my][mx] == 'S' && x >= 0 && x < w && y >= 0 && y < h {
                print!("S");
            } else if cells.contains(&pos) {
                print!("O");
            } else {
                let c = if map[my][mx] == '#' { '#' } else { '.' };
                print!("{}", c);
            }
        }
        println!();
    }
}

fn reachable_count_2_recurrence_relation(start: Pos, max_dist: i32, map: &Vec<Vec<char>>) -> i64 {
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

fn reachable_count_2_fill(start: Pos, max_dist: i32, map: &Vec<Vec<char>>) -> i64 {
    let (h, w) = (map.len() as i32, map[0].len() as i32);
    assert!(h == w);
    let n = h;

    assert!(max_dist % n == n / 2);
    assert!(start.x == n / 2);
    assert!(start.y == n / 2);
    let odd = reachable_count(start, 2 * n + 1, map) as i64;
    let even = reachable_count(start, 2 * n, map) as i64;

    let grid_half_diag = ((max_dist - (n / 2)) / n) as i64;
    let odd_cnt = ((grid_half_diag / 2) * 2 - 1).pow(2);
    let even_cnt = ((grid_half_diag / 2) * 2).pow(2);

    let corner_top = reachable_count(Pos { x: n / 2, y: n - 1 }, n - 1, map) as i64;
    let corner_right = reachable_count(Pos { x: 0, y: n / 2 }, n - 1, map) as i64;
    let corner_bottom = reachable_count(Pos { x: n / 2, y: 0 }, n - 1, map) as i64;
    let corner_left = reachable_count(Pos { x: n - 1, y: n / 2 }, n - 1, map) as i64;

    let small_top_right = reachable_count(Pos { x: 0, y: n - 1 }, n / 2 - 1, map) as i64;
    let small_top_left = reachable_count(Pos { x: n - 1, y: n - 1 }, n / 2 - 1, map) as i64;
    let small_bottom_left = reachable_count(Pos { x: n - 1, y: 0 }, n / 2 - 1, map) as i64;
    let small_bottom_right = reachable_count(Pos { x: 0, y: 0 }, n / 2 - 1, map) as i64;

    let large_top_right = reachable_count(Pos { x: 0, y: n - 1 }, n + n / 2 - 1, map) as i64;
    let large_top_left = reachable_count(Pos { x: n - 1, y: n - 1 }, n + n / 2 - 1, map) as i64;
    let large_bottom_left = reachable_count(Pos { x: n - 1, y: 0 }, n + n / 2 - 1, map) as i64;
    let large_bottom_right = reachable_count(Pos { x: 0, y: 0 }, n + n / 2 - 1, map) as i64;

    return odd_cnt * odd
        + even_cnt * even
        + (corner_top + corner_right + corner_bottom + corner_left)
        + grid_half_diag
            * (small_top_left + small_top_right + small_bottom_left + small_bottom_right)
        + (grid_half_diag - 1)
            * (large_top_left + large_top_right + large_bottom_left + large_bottom_right);
}

fn part_two() {
    let (map, start) = read_input();

    let steps = 26501365;
    let result = reachable_count_2_fill(start, steps, &map);
    println!("{result}");
}

fn main() {
    part_two();
}

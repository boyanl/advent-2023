use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    io::{empty, stdin},
    vec,
};

fn read_input() -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        result.push(line.chars().collect());
    }
    result
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

type Dir = Pos;
const UP: Dir = Dir { x: 0, y: -1 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };

fn get_dir(arrow: char) -> Dir {
    match arrow {
        '>' => RIGHT,
        '^' => UP,
        '<' => LEFT,
        'v' => DOWN,
        _ => todo!(),
    }
}

fn longest_path_len(maze: &Vec<Vec<char>>, start: Pos, end: Pos) -> i32 {
    let (n, m) = (maze.len() as i32, maze[0].len() as i32);
    let mut q: VecDeque<(i32, Pos, Option<Dir>, HashSet<Pos>)> = VecDeque::new();
    q.push_back((0, start, None, HashSet::new()));

    let mut result = 0;
    while !q.is_empty() {
        let (dist, pos, next_dir, mut visited) = q.pop_front().unwrap();
        visited.insert(pos);

        if pos == end {
            if dist > result {
                result = dist;
            }
            continue;
        }

        let dirs = if next_dir.is_some() {
            vec![next_dir.unwrap()]
        } else {
            vec![UP, DOWN, LEFT, RIGHT]
        };

        let mut possible_next = Vec::new();
        for dir in dirs {
            let next = Pos {
                x: pos.x + dir.x,
                y: pos.y + dir.y,
            };
            if next.x >= 0 && next.x < m && next.y >= 0 && next.y < n {
                let c = maze[next.y as usize][next.x as usize];
                if c == '.' {
                    possible_next.push((next, None));
                } else if "^>v<".contains(c) {
                    let dir_next = get_dir(c);
                    possible_next.push((next, Some(dir_next)));
                }
            }
        }

        if possible_next.len() == 1 {
            let (next_pos, forced_dir) = possible_next[0];

            if !visited.contains(&next_pos) {
                q.push_back((dist + 1, next_pos, forced_dir, visited));
            }
        } else {
            for (next_pos, forced_dir) in possible_next {
                if !visited.contains(&next_pos) {
                    let new_visited = visited.clone();
                    q.push_back((dist + 1, next_pos, forced_dir, new_visited));
                }
            }
        }
    }
    result
}

fn part_one() {
    let maze = read_input();
    let start = Pos { x: 1, y: 0 };
    let end = Pos {
        x: (maze[0].len() - 2) as i32,
        y: (maze.len() - 1) as i32,
    };
    assert!(maze[start.y as usize][start.x as usize] == '.');
    assert!(maze[end.y as usize][end.x as usize] == '.');

    let result = longest_path_len(&maze, start, end);
    println!("{result}");
}

fn empty_neighbours(pos: Pos, maze: &Vec<Vec<char>>) -> Vec<Pos> {
    let (n, m) = (maze.len() as i32, maze[0].len() as i32);
    let mut result = Vec::new();

    for dir in [UP, DOWN, LEFT, RIGHT] {
        let next = Pos {
            x: pos.x + dir.x,
            y: pos.y + dir.y,
        };
        if next.x >= 0 && next.x < m && next.y >= 0 && next.y < n {
            let c = maze[next.y as usize][next.x as usize];
            if c != '#' {
                result.push(next);
            }
        }
    }

    result
}

fn find_intersections(
    maze: &Vec<Vec<char>>,
    start: Pos,
    end: Pos,
) -> (Vec<Pos>, Vec<Vec<(usize, i32)>>) {
    let (n, m) = (maze.len() as i32, maze[0].len() as i32);

    let mut intersections = Vec::new();
    intersections.push(start);
    for i in 0..n {
        for j in 0..m {
            let pos = Pos { x: j, y: i };
            if maze[i as usize][j as usize] == '.' && empty_neighbours(pos, maze).len() > 2 {
                intersections.push(pos);
            }
        }
    }
    intersections.push(end);

    let mut neighbours = vec![Vec::new(); intersections.len()];
    for (i, &start) in intersections.iter().enumerate() {
        let mut q = VecDeque::new();
        q.push_back((start, 0));

        let mut visited = HashSet::from([start]);

        while !q.is_empty() {
            let (pos, dist) = q.pop_front().unwrap();

            if pos != start && intersections.contains(&pos) {
                let idx = intersections
                    .iter()
                    .position(|isect| *isect == pos)
                    .unwrap();

                // TODO: Case when there is more than direct path between two intersections is currently unhandled
                // (didn't occur in example or input)
                if neighbours[i].iter().any(|&(to, d)| to == idx) {
                    println!("Possibly snafu");
                }

                neighbours[i].push((idx, dist));
                continue;
            }

            for next in empty_neighbours(pos, maze) {
                if !visited.contains(&next) {
                    q.push_back((next, dist + 1));
                    visited.insert(next);
                }
            }
        }
    }

    (intersections, neighbours)
}

fn add_visited(visited: i64, node: usize) -> i64 {
    return visited | (1 << node);
}

fn contains(visited: i64, node: usize) -> bool {
    return visited & (1 << node) != 0;
}

fn longest_path_len_2(maze: &Vec<Vec<char>>, start: Pos, end: Pos) -> i32 {
    let (intersections, neighbours) = find_intersections(maze, start, end);

    // Number of intersections should be < 64 so we can keep the set of visited nodes in a bitmask
    assert!(intersections.len() < 64);

    let start = 0;
    let end = intersections.len() - 1;

    let mut q = VecDeque::new();
    q.push_back((start, add_visited(0, start), 0));

    let mut result = 0;

    while !q.is_empty() {
        let (node, visited, dist) = q.pop_front().unwrap();

        if node == end {
            result = cmp::max(result, dist);
            continue;
        }

        for &(next, dist_to_next) in &neighbours[node] {
            if !contains(visited, next) {
                q.push_back((next, add_visited(visited, next), dist + dist_to_next));
            }
        }
    }

    result
}

fn part_two() {
    let maze = read_input();
    let start = Pos { x: 1, y: 0 };
    let end = Pos {
        x: (maze[0].len() - 2) as i32,
        y: (maze.len() - 1) as i32,
    };
    assert!(maze[start.y as usize][start.x as usize] == '.');
    assert!(maze[end.y as usize][end.x as usize] == '.');

    let result = longest_path_len_2(&maze, start, end);
    println!("{result}");
}

fn main() {
    part_two();
}

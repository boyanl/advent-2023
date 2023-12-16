use std::{collections::HashSet, io::stdin, ops::Add};

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

type Dir = Pos;

const UP: Dir = Dir { x: 0, y: -1 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };

const DIRS_CLOCKWISE: [Dir; 4] = [UP, RIGHT, DOWN, LEFT];

impl Add<Dir> for Pos {
    type Output = Pos;
    fn add(self, rhs: Dir) -> Self::Output {
        return Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

fn rotate_right(d: Dir) -> Dir {
    let idx = DIRS_CLOCKWISE.iter().position(|el| *el == d).unwrap();
    DIRS_CLOCKWISE[(idx + 1) % 4]
}

fn rotate_left(d: Dir) -> Dir {
    let idx = DIRS_CLOCKWISE.iter().position(|el| *el == d).unwrap();
    DIRS_CLOCKWISE[(idx + 3) % 4]
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Beam {
    pos: Pos,
    dir: Dir,
}

fn inside_bounds(p: Pos, maze: &Vec<Vec<char>>) -> bool {
    let (n, m) = (maze.len(), maze[0].len());

    return p.x >= 0 && p.x < (m as i32) && p.y >= 0 && p.y < (n as i32);
}

fn simulate_beams(maze: &Vec<Vec<char>>, start: Beam) -> HashSet<Pos> {
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();

    let mut beams = Vec::new();
    beams.push(start);

    while !beams.is_empty() {
        let mut new_beams = Vec::new();
        for beam in &beams {
            if visited.contains(beam) {
                continue;
            }
            visited.insert(*beam);

            if beam.pos != start.pos {
                energized.insert(beam.pos);
            }

            let new_pos = beam.pos + beam.dir;
            if !inside_bounds(new_pos, maze) {
                continue;
            }

            match maze[new_pos.y as usize][new_pos.x as usize] {
                '.' => {
                    new_beams.push(Beam {
                        pos: new_pos,
                        dir: beam.dir,
                    });
                }
                '|' => {
                    if beam.dir == LEFT || beam.dir == RIGHT {
                        new_beams.push(Beam {
                            pos: new_pos,
                            dir: UP,
                        });
                        new_beams.push(Beam {
                            pos: new_pos,
                            dir: DOWN,
                        });
                    } else {
                        new_beams.push(Beam {
                            pos: new_pos,
                            dir: beam.dir,
                        });
                    }
                }
                '-' => {
                    if beam.dir == UP || beam.dir == DOWN {
                        new_beams.push(Beam {
                            pos: new_pos,
                            dir: LEFT,
                        });
                        new_beams.push(Beam {
                            pos: new_pos,
                            dir: RIGHT,
                        });
                    } else {
                        new_beams.push(Beam {
                            pos: new_pos,
                            dir: beam.dir,
                        });
                    }
                }
                '\\' => {
                    let new_dir = if beam.dir == UP || beam.dir == DOWN {
                        rotate_left(beam.dir)
                    } else {
                        rotate_right(beam.dir)
                    };
                    new_beams.push(Beam {
                        pos: new_pos,
                        dir: new_dir,
                    });
                }
                '/' => {
                    let new_dir = if beam.dir == UP || beam.dir == DOWN {
                        rotate_right(beam.dir)
                    } else {
                        rotate_left(beam.dir)
                    };
                    new_beams.push(Beam {
                        pos: new_pos,
                        dir: new_dir,
                    });
                }
                _ => todo!(),
            };
        }

        beams = new_beams;
    }

    energized
}

fn read_input() -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        result.push(line.chars().collect());
    }

    result
}

fn show_energized(maze: &Vec<Vec<char>>, energized: &HashSet<Pos>) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            let p = Pos {
                x: j as i32,
                y: i as i32,
            };
            if energized.contains(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
fn part_one() {
    let maze = read_input();
    let energized = simulate_beams(
        &maze,
        Beam {
            pos: Pos { x: -1, y: 0 },
            dir: RIGHT,
        },
    );
    let result = energized.len();

    // show_energized(&maze, &energized);
    println!("{result}");
}

fn part_two() {
    let maze = read_input();
    let (n, m) = (maze.len(), maze[0].len());

    let mut candidate_beams = Vec::new();
    for y in 0..n {
        candidate_beams.push(Beam {
            pos: Pos { x: -1, y: y as i32 },
            dir: RIGHT,
        });
        candidate_beams.push(Beam {
            pos: Pos {
                x: m as i32,
                y: y as i32,
            },
            dir: LEFT,
        });
    }

    for x in 0..m {
        candidate_beams.push(Beam {
            pos: Pos { x: x as i32, y: -1 },
            dir: DOWN,
        });
        candidate_beams.push(Beam {
            pos: Pos {
                x: x as i32,
                y: n as i32,
            },
            dir: UP,
        });
    }

    let result = candidate_beams
        .iter()
        .map(|start| simulate_beams(&maze, *start).len())
        .max()
        .unwrap();
    println!("{result}");
}

fn main() {
    part_two();
}

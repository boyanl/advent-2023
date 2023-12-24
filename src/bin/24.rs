use rug::Float;
use std::{io::stdin, ops::Sub};

use scanf::sscanf;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

type Dir3 = Point3;

impl Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self::Output {
        return Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

fn zero_pt3() -> Point3 {
    return Point3 { x: 0, y: 0, z: 0 };
}

#[derive(Clone, Copy, Debug, Hash)]
struct Point2 {
    x: i64,
    y: i64,
}

#[derive(Clone, Debug)]
struct Point2f {
    x: rug::Float,
    y: rug::Float,
}

type Dir2 = Point2;

fn to_point2(p: Point3) -> Point2 {
    return Point2 { x: p.x, y: p.y };
}

fn read_input() -> Vec<(Point3, Point3)> {
    let mut result = Vec::new();
    for line in stdin().lines().map(|l| l.unwrap()) {
        let (mut sx, mut sy, mut sz, mut vx, mut vy, mut vz) = (0i64, 0i64, 0i64, 0i64, 0i64, 0i64);
        if sscanf!(
            &line,
            "{i64}, {i64}, {i64} @ {i64}, {i64}, {i64}",
            sx,
            sy,
            sz,
            vx,
            vy,
            vz
        )
        .is_ok()
        {
            result.push((
                Point3 {
                    x: sx,
                    y: sy,
                    z: sz,
                },
                Point3 {
                    x: vx,
                    y: vy,
                    z: vz,
                },
            ));
        }
    }

    result
}

type Stone2 = (Point2, Point2);

fn to_f(x: i64) -> rug::Float {
    return rug::Float::with_val(128, x);
}

#[derive(Clone, Debug)]
struct Point3f {
    x: f64,
    y: f64,
}

enum Intersection {
    None,
    Point(Point2f, Float, Float),
    Interval(Float, Float),
}

fn intersect_stones(stone1: Stone2, stone2: Stone2) -> Intersection {
    let (v1, v2) = (stone1.1, stone2.1);
    let (v1x, v1y, v2x, v2y) = (to_f(v1.x), to_f(v1.y), to_f(v2.x), to_f(v2.y));

    let det = v1x.clone() * v2y.clone() - v2x.clone() * v1y.clone();

    let (s1, s2) = (stone1.0, stone2.0);
    let (s1x, s1y, s2x, s2y) = (to_f(s1.x), to_f(s1.y), to_f(s2.x), to_f(s2.y));

    if det == 0 {
        if (v1x.clone() == v2x.clone() && s1x.clone() != s2x.clone())
            || (v1y.clone() == v2y.clone() && s1y.clone() != s2y.clone())
        {
            return Intersection::None;
        }

        let t1_min = (s2x.clone() - s1x.clone()) / v1x.clone();

        if s1y.clone() + t1_min.clone() * v1y.clone() != s2y.clone() {
            return Intersection::None;
        }
        let p = Point2f {
            x: s2x.clone(),
            y: s2y.clone(),
        };

        return Intersection::Interval(t1_min.clone(), -t1_min);
    }

    let t2 = ((s2x.clone() * v1y.clone() - s2y.clone() * v1x.clone())
        - (s1x.clone() * v1y.clone() - s1y.clone() * v1x.clone()))
        / det.clone();
    if v1x == 0 {
        if t2.clone() * v2x.clone() != s1x.clone() - s2x.clone() {
            return Intersection::None;
        }
        return Intersection::Interval(to_f(0), to_f(0));
    }
    let t1 = ((s2x.clone() - s1x.clone()) + t2.clone() * v2x.clone()) / v1x.clone();
    let p = Point2f {
        x: s2x.clone() + t2.clone() * v2x.clone(),
        y: s2y.clone() + t2.clone() * v2y.clone(),
    };
    Intersection::Point(p, t1, t2)
}

fn part_one() {
    let stones = read_input()
        .iter()
        .map(|(pos, dir)| (to_point2(*pos), to_point2(*dir)))
        .collect::<Vec<_>>();

    let min_coord = 200000000000000i64;
    let max_coord = 400000000000000i64;
    // let min_coord = 7;
    // let max_coord = 27;

    let mut result = 0;
    for i in 0..stones.len() {
        for j in i + 1..stones.len() {
            if let Intersection::Point(p, t1, t2) = intersect_stones(stones[i], stones[j]) {
                // println!(
                //     "Stone {i} {:?} an stone {j} {:?} intersect at {:?}; t1 = {t1}, t2 = {t2}",
                //     stones[i], stones[j], p
                // );

                if t1 >= 0.0 && t2 >= 0.0 {
                    if p.x >= to_f(min_coord)
                        && p.x <= to_f(max_coord)
                        && p.y >= to_f(min_coord)
                        && p.y <= to_f(max_coord)
                    {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("{result}");
}

type Stone3 = (Point3, Point3);

fn intersect_stones_2(stone1: Stone3, stone2: Stone3) -> Option<(i64, i64)> {
    let stone1_proj = (to_point2(stone1.0), to_point2(stone1.1));
    let stone2_proj = (to_point2(stone2.0), to_point2(stone2.1));
    let intersection = intersect_stones(stone1_proj, stone2_proj);

    if let Intersection::Point(p, t1, t2) = intersection {
        if !t1.is_integer() || !t2.is_integer() {
            return None;
        }

        let t1_res = t1.to_integer().unwrap().to_i64().unwrap();
        let t2_res = t2.to_integer().unwrap().to_i64().unwrap();

        let z_ok = stone1.0.z + t1_res * stone1.1.z == stone2.0.z + t2_res * stone2.1.z;

        if z_ok {
            return Some((t1_res, t2_res));
        }
    } else if let Intersection::Interval(t1, t2) = intersection {
        let stone1_proj_2 = (
            Point2 {
                x: stone1.0.x,
                y: stone1.0.z,
            },
            Point2 {
                x: stone1.1.x,
                y: stone1.1.z,
            },
        );
        let stone2_proj_2 = (
            Point2 {
                x: stone2.0.x,
                y: stone2.0.z,
            },
            Point2 {
                x: stone2.1.x,
                y: stone2.1.z,
            },
        );
        let intersection2 = intersect_stones(stone1_proj_2, stone2_proj_2);
        if let Intersection::Point(p1, tz1, tz2) = intersection2 {
            if !tz1.is_integer() || !tz2.is_integer() {
                return None;
            }

            let tz1_res = tz1.to_integer().unwrap().to_i64().unwrap();
            let tz2_res = tz2.to_integer().unwrap().to_i64().unwrap();

            let z_ok = stone1.0.z + tz1_res * stone1.1.z == stone2.0.z + tz2_res * stone2.1.z;

            if z_ok {
                return Some((tz1_res, tz2_res));
            }
        } else if let Intersection::Interval(tz1, tz2) = intersection2 {
            let stone1_proj_3 = (
                Point2 {
                    x: stone1.0.y,
                    y: stone1.0.z,
                },
                Point2 {
                    x: stone1.1.y,
                    y: stone1.1.z,
                },
            );
            let stone2_proj_3 = (
                Point2 {
                    x: stone2.0.y,
                    y: stone2.0.z,
                },
                Point2 {
                    x: stone2.1.y,
                    y: stone2.1.z,
                },
            );
            let intersection3 = intersect_stones(stone1_proj_3, stone2_proj_3);
            if let Intersection::Point(p1, tyz1, tyz2) = intersection3 {
                if !tyz1.is_integer() || !tyz2.is_integer() {
                    return None;
                }

                let tyz1_res = tyz1.to_integer().unwrap().to_i64().unwrap();
                let tyz2_res = tyz2.to_integer().unwrap().to_i64().unwrap();

                let z_ok = stone1.0.z + tyz1_res * stone1.1.z == stone2.0.z + tyz2_res * stone2.1.z;

                if z_ok {
                    return Some((tyz1_res, tyz2_res));
                }
            }
        }
    }
    None
}

fn check(stones: &[Stone3], v: Point3) -> Option<Point3> {
    let stones_offset = stones
        .iter()
        .map(|(pi, vi)| (*pi, *vi - v))
        .collect::<Vec<_>>();

    let mut ts = vec![None; stones_offset.len()];
    for i in 0..stones_offset.len() {
        for j in i + 1..stones_offset.len() {
            if let Some((t1, t2)) = intersect_stones_2(stones_offset[i], stones_offset[j]) {
                if (ts[i].is_some() && ts[i].unwrap() != t1)
                    || (ts[j].is_some() && ts[j].unwrap() != t2)
                {
                    return None;
                }
                ts[i] = Some(t1);
                ts[j] = Some(t2);
            } else {
                return None;
            }
        }
    }

    let t0 = ts[0].unwrap();
    let stone0 = stones_offset[0];
    let p = Point3 {
        x: stone0.0.x + t0 * stone0.1.x,
        y: stone0.0.y + t0 * stone0.1.y,
        z: stone0.0.z + t0 * stone0.1.z,
    };

    Some(p)
}

fn part_two() {
    let stones = read_input();

    let r = 300;

    let stones_subset = &stones[0..10];

    let min_x = -r;
    let max_x = r;
    let min_y = -r;
    let max_y = r;
    let min_z = -r;
    let max_z = r;

    'out: for vx in min_x..=max_x {
        for vy in min_y..=max_y {
            for vz in min_z..=max_z {
                let v = Point3 {
                    x: vx,
                    y: vy,
                    z: vz,
                };

                if let Some(res) = check(&stones_subset, v) {
                    println!("{}", res.x + res.y + res.z);
                    break 'out;
                }
            }
        }
    }
}

fn main() {
    part_two();
}

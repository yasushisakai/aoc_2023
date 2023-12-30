use std::{collections::HashMap, vec};

use crate::coordinate::{BBox, Coordinates, Delta, DOWN, LEFT, RIGHT, UP};

type Coord = Coordinates<usize>;
type Connections = HashMap<Coord, Vec<Coord>>;

fn check_insert(
    original: &Coord,
    delta: &Delta,
    bbox: &BBox<usize>,
    connections: &mut Connections,
) {
    if let Some(new_coord) = bbox.check_within(original, delta) {
        connections
            .entry(original.clone())
            .or_insert(Vec::new())
            .push(new_coord);
    }
}

fn get_next(current: &Coord, prev: &Coord, connections: &Connections) -> Option<Coord> {
    let Some(directions) = connections.get(current) else {
        return None;
    };

    let Some(pos) = directions.iter().position(|c| c == prev) else {
        return None;
    };

    let dir = if pos == 0 {
        directions[1].clone()
    } else {
        directions[0].clone()
    };

    Some(dir)
}

fn parse(
    content: &str,
) -> (
    Coord,
    BBox<usize>,
    HashMap<Coord, Vec<Coord>>,
    HashMap<Coord, char>,
) {
    let mut connections = HashMap::new();
    let mut lookup = HashMap::new();
    let mut start = Coordinates::new(0, 0);

    let max_y = content.lines().count();
    let max_x = content.lines().next().unwrap().chars().count();

    let bbox = BBox::new(max_x, max_y);

    for (y, line) in content.lines().enumerate() {
        for (x, mark) in line.chars().enumerate() {
            let from = Coordinates::new(x, y);
            lookup.insert(from.clone(), mark);
            match mark {
                '.' => continue,
                'S' => start = from,
                '|' => {
                    check_insert(&from, &UP, &bbox, &mut connections);
                    check_insert(&from, &DOWN, &bbox, &mut connections);
                }
                '-' => {
                    check_insert(&from, &LEFT, &bbox, &mut connections);
                    check_insert(&from, &RIGHT, &bbox, &mut connections);
                }
                'F' => {
                    check_insert(&from, &RIGHT, &bbox, &mut connections);
                    check_insert(&from, &DOWN, &bbox, &mut connections);
                }
                'L' => {
                    check_insert(&from, &UP, &bbox, &mut connections);
                    check_insert(&from, &RIGHT, &bbox, &mut connections);
                }
                'J' => {
                    check_insert(&from, &UP, &bbox, &mut connections);
                    check_insert(&from, &LEFT, &bbox, &mut connections);
                }
                '7' => {
                    check_insert(&from, &LEFT, &bbox, &mut connections);
                    check_insert(&from, &DOWN, &bbox, &mut connections);
                }
                _ => continue,
            }
        }
    }

    (start, bbox, connections, lookup)
}

pub fn part1(content: &str) -> usize {
    let (start, bbox, connections, _) = parse(content);

    let mut steps = 1;
    let mut prev = start.clone();

    let neighbors = vec![UP, DOWN, LEFT, RIGHT];

    let neighbor = neighbors
        .iter()
        .find(|n| {
            let Some(c) = bbox.check_within(&prev, &n) else {
                return false;
            };
            get_next(&c, &prev, &connections).is_some()
        })
        .unwrap();

    let mut current = bbox.check_within(&prev, &neighbor).unwrap();

    loop {
        let n = get_next(&current, &prev, &connections);

        let Some(next) = n else {
            panic!("not connected");
        };

        if next == start {
            steps += 1;
            break;
        }

        prev = current;
        current = next;
        steps += 1;
    }

    steps / 2
}

pub fn part2(content: &str, is_s_pipe: bool) -> u32 {
    let (start, bbox, connections, lookup) = parse(content);

    let mut prev = start.clone();
    let mut path = vec![start.clone()];

    let neighbors = vec![UP, DOWN, LEFT, RIGHT];

    let neighbor = neighbors
        .iter()
        .find(|n| {
            let Some(c) = bbox.check_within(&prev, &n) else {
                return false;
            };
            get_next(&c, &prev, &connections).is_some()
        })
        .unwrap();

    let mut current = bbox.check_within(&prev, &neighbor).unwrap();

    loop {
        path.push(current.clone());
        let n = get_next(&current, &prev, &connections);

        let Some(next) = n else {
            panic!("not connected");
        };

        prev = current;

        if next == start {
            break;
        }

        current = next;
    }

    let mut nest = 0;

    for y in 0..bbox.y_max {
        let mut is_inside = false;
        for x in 0..bbox.x_max {
            let coord = Coord::new(x, y);
            let is_loop = path.contains(&coord);
            let mark = lookup.get(&coord).unwrap();
            if is_loop {
                match mark {
                    'L' | 'J' | '|' => is_inside = !is_inside,
                    'S' => {
                        if is_s_pipe {
                            is_inside = !is_inside
                        }
                    }
                    _ => (),
                }
            } else {
                if is_inside {
                    nest += 1;
                } else {
                }
            }
        }
    }

    nest
}

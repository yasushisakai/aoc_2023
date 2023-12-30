use std::collections::{HashMap, HashSet};

fn to_coord(s: &str) -> (usize, usize, usize) {
    let mut sp = s.split(',');
    let x = sp.next().unwrap().parse().unwrap();
    let y = sp.next().unwrap().parse().unwrap();
    let z = sp.next().unwrap().parse().unwrap();
    (x, y, z)
}

type Coord = (usize, usize, usize);
type Brick = (Coord, Coord);

pub fn disintegrate(
    bid: &usize,
    holds: &HashMap<usize, Vec<usize>>,
    depends: &HashMap<usize, Vec<usize>>,
    falls: &mut HashSet<usize>,
) {
    falls.insert(*bid);

    let Some(held) = holds.get(bid) else {
        return;
    };

    for h in held {
        if falls.contains(&h) {
            continue;
        }

        let held_depends_on = depends.get(h).unwrap();

        let remain = held_depends_on
            .iter()
            .filter(|hd| !falls.contains(hd))
            .count();

        if remain == 0 {
            disintegrate(&h, holds, depends, falls);
        }
    }
}

pub fn drop(
    space: &mut HashMap<Coord, usize>,
    // key is below, values are whats on top
    holds: &mut HashMap<usize, Vec<usize>>,
    // key is top, values are whats on under
    on: &mut HashMap<usize, Vec<usize>>,
    brick: &Brick,
    brick_id: usize,
) -> Brick {
    let ((sx, sy, sz), (ex, ey, ez)) = brick;

    if *sz == 1 {
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                for z in *sz..=*ez {
                    space.insert((x, y, z), brick_id);
                }
            }
        }
        return *brick;
    }

    let mut z = *sz;
    let dz = ez - sz;

    while z > 1 {
        let mut occupied = false;
        for x in *sx..=*ex {
            for y in *sy..=*ey {
                if space.contains_key(&(x, y, z - 1)) {
                    let below = *space.get(&(x, y, z - 1)).unwrap();
                    holds.entry(below).or_default().push(brick_id);
                    on.entry(brick_id).or_default().push(below);
                    occupied = true;
                }
            }
        }
        if occupied {
            break;
        } else {
            z -= 1;
        }
    }

    for x in *sx..=*ex {
        for y in *sy..=*ey {
            for zz in z..=(z + dz) {
                space.insert((x, y, zz), brick_id);
            }
        }
    }

    ((*sx, *sy, z), (*ex, *ey, z + dz))
}

pub fn part1(content: &str) -> usize {
    let mut bricks: Vec<_> = content
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start = to_coord(start);
            let end = to_coord(end);
            (start, end)
        })
        .collect();

    bricks.sort_by_key(|(start, _end)| start.2);

    let mut space = HashMap::new();
    let mut holds: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut depends: HashMap<usize, Vec<usize>> = HashMap::new();

    let bricks: Vec<_> = bricks
        .iter()
        .enumerate()
        .map(|(i, b)| drop(&mut space, &mut holds, &mut depends, b, i))
        .collect();

    let mut free = 0;
    for i in 0..bricks.len() {
        let Some(holding) = holds.get(&i) else {
            free += 1;
            continue;
        };

        let mut others = true;
        for h in holding {
            let mut on_top_of = depends.get(h).unwrap().clone();
            on_top_of.retain(|&x| x != i);
            if on_top_of.is_empty() {
                others = false;
                break;
            }
        }

        if others {
            free += 1;
        }
    }

    free
}

pub fn part2(content: &str) -> usize {
    let mut bricks: Vec<_> = content
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let start = to_coord(start);
            let end = to_coord(end);
            (start, end)
        })
        .collect();

    bricks.sort_by_key(|(start, _end)| start.2);

    let mut space = HashMap::new();
    let mut holds: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut depends: HashMap<usize, Vec<usize>> = HashMap::new();

    let bricks: Vec<_> = bricks
        .iter()
        .enumerate()
        .map(|(i, b)| drop(&mut space, &mut holds, &mut depends, b, i))
        .collect();

    (0..bricks.len())
        .map(|i| {
            let mut falls = HashSet::new();
            disintegrate(&i, &holds, &depends, &mut falls);
            println!("brick: {i}, {}", falls.len());
            falls.len() - 1
        })
        .sum()
}

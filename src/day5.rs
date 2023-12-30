use std::{sync::mpsc::channel, thread};

fn seeds_single(line: &str) -> Vec<u64> {
    line.split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|e| !e.is_empty())
        .map(|e| e.parse::<u64>().unwrap())
        .collect()
}

pub fn parse_maps<'a, I>(lines: &mut I) -> Vec<Vec<(u64, u64, u64)>>
where
    I: Iterator<Item = &'a str>,
{
    let mut parent = Vec::new();
    let mut children = Vec::new();

    // parse mappings
    for line in lines {
        if line.is_empty() {
            children.sort_by(|(_, a, _): &(u64, u64, u64), &(_, b, _)| a.cmp(&b));

            parent.push(children.clone());
            children.clear();
        }

        let mut map = line.split(' ');

        let Ok(destination) = map.next().unwrap().parse::<u64>() else {
            continue;
        };

        let source = map.next().unwrap().parse::<u64>().unwrap();
        let length = map.next().unwrap().parse::<u64>().unwrap();

        children.push((destination, source, length));
    }

    children.sort_by(|(_, a, _): &(u64, u64, u64), &(_, b, _)| a.cmp(&b));
    parent.push(children);

    parent
}

fn parse_maps_rev<'a, I>(lines: &mut I) -> Vec<Vec<(u64, u64, u64)>>
where
    I: Iterator<Item = &'a str>,
{
    let mut parent = Vec::new();
    let mut children = Vec::new();

    // parse mappings
    for line in lines {
        if line.is_empty() {
            children.sort_by(|(a, _, _): &(u64, u64, u64), &(b, _, _)| a.cmp(&b));
            parent.push(children.clone());
            children.clear();
        }

        let mut map = line.split(' ');

        let Ok(destination) = map.next().unwrap().parse::<u64>() else {
            continue;
        };

        let source = map.next().unwrap().parse::<u64>().unwrap();
        let length = map.next().unwrap().parse::<u64>().unwrap();

        children.push((destination, source, length));
    }

    children.sort_by(|(a, _, _): &(u64, u64, u64), &(b, _, _)| a.cmp(&b));
    parent.push(children);
    parent.reverse();
    parent
}

fn location_to_seed(location: u64, map_rev: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut v = location;
    for child in map_rev.iter() {
        for (dest, source, length) in child {
            if v >= *dest && v < dest + length {
                let delta = v - dest;
                v = *source + delta;
                break;
            }

            if v < *dest {
                break;
            }
        }
    }

    v
}

fn seed_to_location(seed: &u64, map: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut v = *seed;
    for child in map.iter() {
        for (dest, source, length) in child {
            if v >= *source && v < source + length {
                let delta = v - source;
                v = *dest + delta;
                break;
            }

            if v < *source {
                break;
            }
        }
    }

    v
}

fn lowest_seed(seeds: Vec<u64>, map: Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let locations = seeds.iter().map(|seed| seed_to_location(&seed, &map));
    locations.min().unwrap()
}

pub fn part1(content: &str) -> u64 {
    let mut lines = content.lines();
    let seeds: Vec<u64> = seeds_single(lines.next().unwrap());
    lines.next().unwrap();
    let parent = parse_maps(&mut lines);
    lowest_seed(seeds, parent)
}

fn seed_ranges(line: &str) -> Vec<(u64, u64)> {
    let mut numbers = line
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|e| !e.is_empty())
        .map(|e| e.parse::<u64>().unwrap());

    let mut seeds: Vec<(u64, u64)> = Vec::new();

    while let Some(start) = numbers.next() {
        let length = numbers.next().unwrap();
        seeds.push((start, length));
    }

    seeds.sort_by(|(a, _): &(u64, u64), (b, _)| a.cmp(&b));

    seeds
}

fn is_in_range(seed: u64, ranges: &Vec<(u64, u64)>) -> bool {
    for (start, length) in ranges {
        if seed >= *start && seed < (start + length) {
            return true;
        }

        if seed < *start {
            return false;
        }
    }

    false
}

pub fn part2(content: &str) -> u64 {
    let mut lines = content.lines();

    let ranges = seed_ranges(lines.next().unwrap());
    lines.next().unwrap();

    let parent = parse_maps_rev(&mut lines);

    let (tx, rx) = channel::<u64>();

    let threads = 16;

    for i in 0..threads {
        let tx = tx.clone();
        let parent_cl = parent.clone();
        let ranges_cl = ranges.clone();
        thread::spawn(move || {
            for n in 0u64.. {
                let num = n * threads + i;
                let seed = location_to_seed(num, &parent_cl);
                if is_in_range(seed, &ranges_cl) {
                    tx.send(num).unwrap();
                    return;
                }
            }
        });
    }

    let mut lowest = u64::MAX;
    for _ in 0..threads {
        let l = rx.recv().unwrap();
        if lowest > l {
            lowest = l;
        }
    }

    lowest
}

use std::collections::HashMap;

use itertools::Itertools;

pub fn part1(content: &str) -> usize {
    let max_unit_load = content.lines().count();
    let cols = content.lines().next().unwrap().len();
    let mut transposed = vec![String::new(); cols];

    for line in content.lines() {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }

    let rocks = transposed
        .iter()
        .map(|l| {
            let mut squares = l
                .chars()
                .enumerate()
                .filter(|(_i, c)| c == &'#')
                .map(|(i, _)| max_unit_load - (i + 1))
                .collect::<Vec<usize>>();
            squares.insert(0, max_unit_load);
            let between = l.split('#').collect();
            (squares, between)
        })
        .collect::<Vec<(Vec<usize>, Vec<&str>)>>();

    let areas = rocks
        .iter()
        .map(|(squares, between)| {
            let mut line_total = 0;
            for (pos, b) in squares.iter().zip(between) {
                let num_rocks = b.chars().filter(|c| c == &'O').count();
                let rect = pos * num_rocks;
                let triangle: usize = (0usize..num_rocks).sum();
                line_total += rect - triangle;
            }
            line_total
        })
        .sum();

    areas
}

pub fn north_and_west(content: Vec<String>) -> Vec<String> {
    let transposed = transpose(content);

    let rocks = transposed
        .iter()
        .map(|l| {
            l.split('#')
                .map(|s| {
                    let num_rocks = s.chars().filter(|c| c == &'O').count();
                    let spots = vec!['.'; s.len()].into_iter().collect::<String>();
                    let new_pos = spots.replacen('.', "O", num_rocks);
                    new_pos
                })
                .join("#")
        })
        .collect::<Vec<String>>();

    rocks
}

pub fn south_and_east(content: Vec<String>) -> Vec<String> {
    let transposed = transpose(content);

    let rocks = transposed
        .iter()
        .map(|l| {
            l.split('#')
                .map(|s| {
                    let num_rocks = s.chars().filter(|c| c == &'O').count();
                    let spots = vec!['.'; s.len()].into_iter().collect::<String>();
                    let new_pos = spots.replacen('.', "O", num_rocks);
                    let new_pos = new_pos.chars().rev().collect::<String>();
                    new_pos
                })
                .join("#")
        })
        .collect::<Vec<String>>();

    rocks
}

pub fn cycle(content: Vec<String>) -> Vec<String> {
    let north = north_and_west(content);
    let west = north_and_west(north);
    let south = south_and_east(west);
    let east = south_and_east(south);

    east
}

pub fn transpose(content: Vec<String>) -> Vec<String> {
    let len = content.iter().next().unwrap().len();
    let mut new_transposed = vec![String::new(); len];

    for line in content.iter() {
        for (i, c) in line.chars().enumerate() {
            new_transposed[i].push(c);
        }
    }

    new_transposed
}

pub fn collect_sum(pattern: Vec<String>) -> usize {
    println!("{:?}", pattern);
    let max = pattern.len();

    pattern.iter().enumerate().fold(0, |acc, (i, v)| {
        let points = max - i;
        let count = v.chars().filter(|m| m == &'O').count();
        acc + count * points
    })
}

pub fn part2(content: &str) -> usize {
    let mut v: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    let mut history = HashMap::new();

    let mut i = 0;

    let total = 1000_000_000;

    while i < 1000_000_000 {
        v = cycle(v);
        // what was i when we last saw the same image?
        let Some(last) = history.get(&v) else {
            history.insert(v.clone(), i);
            i += 1;
            continue;
        };

        let diff = i - last;

        if i + diff < total {
            let jump = (((total - i) / diff) - 1) * diff + 1;
            i += jump;
            continue;
        }

        i += 1;
    }

    collect_sum(v)
}

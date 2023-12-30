use std::collections::HashMap;

fn get_instructions(line: &str) -> Vec<usize> {
    line.chars()
        .map(|c| match c {
            'R' => 1,
            'L' => 0,
            _ => panic!(),
        })
        .collect()
}

pub fn part1(content: &str) -> u32 {
    let mut lines = content.lines();
    let instructions = get_instructions(lines.next().unwrap());
    lines.next().unwrap();

    let map: HashMap<&str, Vec<&str>> = lines
        .map(|line| {
            let (key, values) = line.split_once('=').unwrap();
            let key = key.trim();
            let values: Vec<&str> = values
                .trim()
                .trim_matches(|c| c == '(' || c == ')' || c == ' ')
                .split(',')
                .map(|s| s.trim())
                .collect();
            (key, values)
        })
        .collect();

    let mut current_node = "AAA";

    for (i, inst) in instructions.iter().cycle().enumerate() {
        let next_values = map.get(current_node).unwrap();
        let next_node = next_values[*inst];
        if next_node == "ZZZ" {
            return (i + 1) as u32;
        }
        current_node = next_node;
    }

    0
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    a * (b / g)
}

pub fn part2(content: &str) -> u64 {
    let mut lines = content.lines();
    let instructions = get_instructions(lines.next().unwrap());
    lines.next().unwrap();

    let map: HashMap<&str, Vec<&str>> = lines
        .map(|line| {
            let (key, values) = line.split_once('=').unwrap();
            let key = key.trim();
            let values: Vec<&str> = values
                .trim()
                .trim_matches(|c| c == '(' || c == ')' || c == ' ')
                .split(',')
                .map(|s| s.trim())
                .collect();
            (key, values)
        })
        .collect();

    let current_nodes: Vec<&str> = map
        .iter()
        .filter(|(k, _v)| k.ends_with('A'))
        .map(|(k, _v)| *k)
        .collect();

    // let mut new_current_nodes: Vec<&str> = Vec::new();

    // this goes in cycles;
    let first_sighted: Vec<usize> = current_nodes
        .iter()
        .map(|node| {
            let mut current = *node;
            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(i, instruction)| {
                    let next = map.get(current).unwrap()[*instruction];
                    if next.ends_with('Z') {
                        // new_current_nodes.push(next);
                        Some(i + 1)
                    } else {
                        current = next;
                        None
                    }
                })
                .unwrap()
        })
        .collect();

    // let second_sighted: Vec<usize> = new_current_nodes
    //     .iter()
    //     .map(|node| {
    //         let mut current = *node;
    //         instructions
    //             .iter()
    //             .cycle()
    //             .enumerate()
    //             .find_map(|(i, instruction)| {
    //                 let next = map.get(current).unwrap()[*instruction];
    //                 if next.ends_with('Z') {
    //                     Some(i + 1)
    //                 } else {
    //                     current = next;
    //                     None
    //                 }
    //             })
    //             .unwrap()
    //     })
    //     .collect();

    first_sighted.iter().fold(1, |acc, v| {
        let l = lcm(acc, *v);
        l
    }) as u64
}

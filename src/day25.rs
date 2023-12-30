use rand::seq::SliceRandom;
use std::collections::{HashMap, VecDeque};

fn path<'a>(
    start: &'a str,
    end: &'a str,
    connections: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<&'a str> {
    let mut q = VecDeque::new();
    q.push_back((start, vec![start]));

    while let Some((p, v)) = q.pop_front() {
        if p == end {
            return v;
        }

        let neigbors = connections.get(&p).unwrap();

        for n in neigbors {
            if v.contains(n) {
                continue;
            }
            let mut visited = v.clone();
            visited.push(n);
            q.push_back((n, visited));
        }
    }

    panic!()
}

fn reachable(p: &str, connections: &HashMap<&str, Vec<&str>>) -> usize {
    let mut q = VecDeque::new();
    let mut visited = vec![p];
    q.push_back(p);

    while let Some(p) = q.pop_front() {
        let neigbors = connections.get(&p).unwrap();

        for n in neigbors {
            if visited.contains(n) {
                continue;
            }
            visited.push(n);
            q.push_back(n);
        }
    }

    visited.len()
}

pub fn part1(content: &str) -> usize {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in content.lines() {
        let (s, d) = line.split_once(": ").unwrap();
        for dest in d.split(" ") {
            connections.entry(s).or_default().push(dest);
            connections.entry(dest).or_default().push(s);
        }
    }

    let nodes: Vec<_> = connections.keys().collect();
    let mut rng = &mut rand::thread_rng();

    let test_iterations = 200;
    let mut node_ranking: HashMap<(&str, &str), usize> = HashMap::new();

    for _i in 0..test_iterations {
        let pick: Vec<_> = nodes.choose_multiple(&mut rng, 2).collect();
        let visited = path(pick[0], pick[1], &connections);
        for v in visited.windows(2) {
            let a;
            let b;
            if v[0] < v[1] {
                a = v[0];
                b = v[1];
            } else {
                a = v[1];
                b = v[0];
            }
            node_ranking
                .entry((&a, &b))
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
    }

    let mut top = node_ranking
        .iter()
        .map(|(n, f)| (*f, *n))
        .collect::<Vec<(usize, (&str, &str))>>();
    top.sort();
    top.reverse();

    for (_, (a, b)) in &top[..3] {
        let an = connections.get_mut(a).unwrap();
        an.retain(|n| n != b);
        let bn = connections.get_mut(b).unwrap();
        bn.retain(|n| n != a);
    }

    let a = top[0].1 .0;
    let b = top[0].1 .1;
    let part_a = reachable(a, &connections);
    let part_b = reachable(b, &connections);

    part_a * part_b
}

pub fn part2(content: &str) -> usize {
    todo!()
}

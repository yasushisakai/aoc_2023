use std::collections::{HashMap, HashSet};
type Coord = (isize, isize);

type Graph = HashMap<Coord, Vec<(Coord, usize)>>;

fn parse_graph(content: &str) -> (Coord, Graph) {
    let mut graph: Graph = HashMap::new();
    let w = content.lines().next().unwrap().len();
    let h = content.lines().count();

    let goal = (w as isize - 2, h as isize - 1);

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;
            }

            let (px, py) = (x as isize, y as isize);

            let directions = match c {
                '.' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
                '>' => vec![(1, 0)],
                '<' => vec![(-1, 0)],
                '^' => vec![(0, -1)],
                'v' => vec![(0, 1)],
                _ => panic!("invalid character"),
            };

            let mut neighbors = Vec::new();

            for (dx, dy) in directions {
                let (nx, ny) = (px + dx, py + dy);
                // boundary check
                if nx < 0 || nx >= w as isize || ny < 0 || ny >= h as isize {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);

                let nc = content.lines().nth(ny).unwrap().chars().nth(nx).unwrap();
                match (nc, dx, dy) {
                    ('#', _, _) => continue,
                    // ('>', -1, _) => continue,
                    // ('<', 1, _) => continue,
                    // ('^', _, 1) => continue,
                    // ('v', _, -1) => continue,
                    _ => (),
                }
                neighbors.push(((nx as isize, ny as isize), 1));
            }

            if !neighbors.is_empty() {
                graph.insert((px, py), neighbors);
            }
        }
    }

    // contraction
    let pass_throughs: Vec<Coord> = graph
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(c, _)| *c)
        .collect();

    for pass in pass_throughs {
        let neighbors = graph.remove(&pass).unwrap();
        let (a, ad) = neighbors[0];
        let (b, bd) = neighbors[1];

        let a_neighbors = graph.get_mut(&a).unwrap();
        if let Some(i) = a_neighbors.iter().position(|(p, _)| p == &pass) {
            a_neighbors[i] = (b, ad + bd);
        }
        let b_neighbors = graph.get_mut(&b).unwrap();
        if let Some(i) = b_neighbors.iter().position(|(p, _)| p == &pass) {
            b_neighbors[i] = (a, ad + bd);
        }
    }

    (goal, graph)
}

fn dfs(p: &Coord, goal: &Coord, graph: &Graph, visited: &HashSet<Coord>) -> Option<usize> {
    if p == goal {
        return Some(0);
    }

    let mut visited = visited.clone();
    visited.insert(*p);

    let neighbors = graph.get(p).unwrap();

    let mut dist = 0;
    for (n, nd) in neighbors {
        if visited.contains(n) {
            continue;
        }

        if let Some(sum_d) = dfs(n, goal, graph, &visited) {
            dist = dist.max(nd + sum_d);
        }
    }

    if dist == 0 {
        None
    } else {
        Some(dist)
    }
}

pub fn part1(content: &str) -> usize {
    let (goal, graph) = parse_graph(content);
    let steps = dfs(&(1, 0), &goal, &graph, &HashSet::new());
    steps.unwrap()
}

pub fn part2(content: &str) -> usize {
    let new_content = content.replace(&['>', '<', '^', 'v'], ".");
    let (goal, graph) = parse_graph(&new_content);
    let steps = dfs(&(1, 0), &goal, &graph, &HashSet::new());
    steps.unwrap()
}

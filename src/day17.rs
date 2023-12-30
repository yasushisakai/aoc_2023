use std::collections::{BinaryHeap, HashMap};

fn dijkstra(grid: &Grid, min_move: isize, max_move: isize) -> isize {
    let mut q = BinaryHeap::new();
    let mut costs: HashMap<Node, isize> = HashMap::new();

    let start = Node {
        loc: (0, 0),
        direction: (0, 0),
    };
    q.push((0, start));

    while let Some((cost, n)) = q.pop() {
        if n.loc == grid.end() {
            return -cost;
        }

        let (x, y) = n.loc;

        for (ndx, ndy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let reverse = (ndx * -1, ndy * -1);

            if n.direction == (ndx, ndy) || n.direction == reverse {
                continue;
            }

            let mut new_cost = -cost;

            for m in 1..=max_move {
                let nx = ndx * m + x;
                let ny = ndy * m + y;

                if !grid.in_bound((nx, ny)) {
                    continue;
                }

                new_cost += grid.get((nx as usize, ny as usize)) as isize;

                if m < min_move {
                    continue;
                }

                let node = Node {
                    loc: (nx, ny),
                    direction: (ndx, ndy),
                };

                if let Some(past_cost) = costs.get(&node) {
                    // there was a node already
                    if past_cost > &new_cost {
                        costs.insert(node.clone(), new_cost);
                        q.push((-new_cost, node));
                    }
                } else {
                    // new node!
                    costs.insert(node.clone(), new_cost);
                    q.push((-new_cost, node))
                }
            }
        }
    }

    panic!("should not return here.")
}

type Coord = (isize, isize);

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
struct Node {
    // cost: isize, // for binary heap's sorting
    loc: Coord,
    direction: (isize, isize),
}

struct Grid {
    numbers: Vec<Vec<usize>>,
    w: usize,
    h: usize,
}

impl Grid {
    fn parse(content: &str) -> Self {
        let numbers: Vec<Vec<usize>> = content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect();

        let h = numbers.len();
        let w = numbers[0].len();

        Self { numbers, h, w }
    }

    fn end(&self) -> Coord {
        (self.w as isize - 1, self.h as isize - 1)
    }

    fn get(&self, (x, y): (usize, usize)) -> usize {
        self.numbers[y][x]
    }

    fn in_bound(&self, (x, y): Coord) -> bool {
        !(x < 0 || y < 0 || x > self.w as isize - 1 || y > self.h as isize - 1)
    }
}

pub fn part1(content: &str) -> isize {
    let grid = Grid::parse(content);
    dijkstra(&grid, 0, 3)
}

pub fn part2(content: &str) -> isize {
    let grid = Grid::parse(content);
    dijkstra(&grid, 4, 10)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    V,
    H,
}

#[derive(Debug)]
pub struct Pattern {
    lines: Vec<String>,
    width: usize,
    height: usize,
}

impl Pattern {
    pub fn new(lines: &Vec<String>) -> Self {
        let height = lines.len();
        let width = lines.iter().next().unwrap().len();
        Self {
            lines: lines.clone(),
            height,
            width,
        }
    }

    pub fn parse(pattern: &str) -> Self {
        let lines: Vec<String> = pattern.lines().map(|l| l.to_string()).collect();
        let height = lines.len();
        let width = lines.iter().next().unwrap().len();
        Self {
            lines,
            width,
            height,
        }
    }

    pub fn row(&self, n: usize) -> String {
        self.lines[n].clone()
    }

    pub fn col(&self, n: usize) -> String {
        self.lines
            .iter()
            .map(|l| l.chars().nth(n).unwrap().to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn row_off_by(&self, i: usize, j: usize) -> usize {
        let a = self.row(i);
        let b = self.row(j);

        a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
    }

    pub fn col_off_by(&self, i: usize, j: usize) -> usize {
        let a = self.col(i);
        let b = self.col(j);

        a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
    }

    pub fn find_offby(&self, limit: usize) -> (Direction, usize) {
        // start with horizontal from top
        let bottom = self.height - 1;
        for y in (1..bottom).step_by(2) {
            let mut total_off = self.row_off_by(y, bottom);
            if total_off < limit + 1 {
                let check = y.abs_diff(bottom) / 2;
                for mi in 1..=check {
                    let ni = y + mi;
                    let nj = bottom - mi;
                    total_off += self.row_off_by(ni, nj);
                    if total_off > limit {
                        break;
                    }
                }
            } // break
            if total_off == limit {
                let r = y + y.abs_diff(bottom) / 2;
                return (Direction::H, r + 1);
            }
        }

        // start with horizontal from the bottom
        for y in (1..bottom).rev().step_by(2) {
            let mut total_off = self.row_off_by(y, 0);
            if total_off < limit + 1 {
                for mi in 1..=(y / 2) {
                    let nj = y - mi;
                    total_off += self.row_off_by(mi, nj);
                    if total_off > limit {
                        break;
                    }
                }
            } // break
            if total_off == limit {
                let r = y / 2;
                return (Direction::H, r + 1);
            }
        }

        // vertical from left
        let right = self.width - 1;
        for x in (1..right).step_by(2) {
            let mut total_off = self.col_off_by(x, right);
            if total_off < limit + 1 {
                let check = x.abs_diff(right) / 2;
                for mi in 1..=check {
                    let ni = x + mi;
                    let nj = right - mi;
                    total_off += self.col_off_by(ni, nj);
                    if total_off > limit {
                        break;
                    }
                }
            } // break

            if total_off == limit {
                let r = x + x.abs_diff(right) / 2;
                return (Direction::V, r + 1);
            }
        }

        // vertical from right
        for x in (1..right).rev().step_by(2) {
            let mut total_off = self.col_off_by(x, 0);
            if total_off < limit + 1 {
                for mi in 1..=x / 2 {
                    let nj = x - mi;
                    total_off += self.col_off_by(mi, nj);
                    if total_off > limit {
                        break;
                    }
                }
            } // break

            if total_off == limit {
                let r = x / 2;
                return (Direction::V, r + 1);
            }
        }

        panic!("should find a mirror");
    }
}

fn parse_patterns(content: &str) -> Vec<Pattern> {
    let mut partial = Vec::new();
    let mut patterns = Vec::new();

    for line in content.lines() {
        if line.is_empty() {
            let pattern = Pattern::new(&partial);
            patterns.push(pattern);
            partial.clear();
            continue;
        }
        partial.push(line.to_string());
    }

    let pattern = Pattern::new(&partial);
    patterns.push(pattern);

    patterns
}

pub fn part1(content: &str) -> usize {
    let patterns = parse_patterns(content);

    patterns
        .iter()
        .map(|p| {
            let (d, ln) = p.find_offby(0);
            match d {
                Direction::V => ln,
                Direction::H => ln * 100,
            }
        })
        .sum()
}

pub fn part2(content: &str) -> usize {
    let patterns = parse_patterns(content);

    patterns
        .iter()
        .map(|p| {
            let (d, ln) = p.find_offby(1);
            match d {
                Direction::V => ln,
                Direction::H => ln * 100,
            }
        })
        .sum()
}

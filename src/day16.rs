use rayon::prelude::*;

pub struct Contraption {
    lines: Vec<String>,
    w: i16,
    h: i16,
}

type Coord = (i16, i16);

impl Contraption {
    pub fn parse(content: &str) -> Self {
        let lines = content
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let w = lines.first().unwrap().len() as i16;
        let h = lines.len() as i16;

        Self { lines, w, h }
    }

    pub fn get(&self, (x, y): Coord) -> char {
        self.lines[y as usize].chars().nth(x as usize).unwrap()
    }

    pub fn top(&self) -> Vec<Coord> {
        (0..self.w).map(|x| (x as i16, 0)).collect()
    }

    pub fn bottom(&self) -> Vec<Coord> {
        (0..self.w).map(|x| (x as i16, self.h - 1)).collect()
    }

    pub fn left(&self) -> Vec<Coord> {
        (0..self.h).map(|y| (0, y as i16)).collect()
    }

    pub fn right(&self) -> Vec<Coord> {
        (0..self.h).map(|y| (self.w - 1, y as i16)).collect()
    }
    pub fn out_of_bounds(&self, (x, y): Coord) -> bool {
        x < 0 || y < 0 || x >= self.w || y >= self.h
    }

    pub fn next(&self, (x, y): Coord, next_dir: &Direction) -> Coord {
        match next_dir {
            Direction::Right => (x - 1, y),
            Direction::Left => (x + 1, y),
            Direction::Above => (x, y + 1),
            Direction::Below => (x, y - 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub enum Direction {
    Above,
    Below,
    Left,
    Right,
}

impl Direction {
    fn reflect_back_slash(&self) -> Self {
        match self {
            Direction::Right => Direction::Below,
            Direction::Left => Direction::Above,
            Direction::Above => Direction::Left,
            Direction::Below => Direction::Right,
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Above => Direction::Below,
            Direction::Below => Direction::Above,
        }
    }

    fn reflect_forward_slash(&self) -> Self {
        let back = self.reflect_back_slash();
        back.reverse()
    }

    fn split_pipe(&self) -> Vec<Self> {
        match self {
            Direction::Right | Direction::Left => vec![Direction::Below, Direction::Above],
            a @ _ => vec![a.clone()],
        }
    }

    fn split_minus(&self) -> Vec<Self> {
        match self {
            Direction::Below | Direction::Above => vec![Direction::Right, Direction::Left],
            a @ _ => vec![a.clone()],
        }
    }
}

pub fn step(
    pos: (i16, i16),
    from: Direction,
    con: &Contraption,
    visited: &mut Vec<(Coord, Direction)>,
) {
    if con.out_of_bounds(pos) {
        return;
    }

    if visited.contains(&(pos, from)) {
        return;
    }

    if !visited.iter().any(|(p, _)| p == &pos) {
        visited.push((pos, from));
    }

    let next_dirs = match con.get(pos) {
        '.' => vec![from],
        '\\' => vec![from.reflect_back_slash()],
        '/' => vec![from.reflect_forward_slash()],
        '|' => {
            let v = from.split_pipe();
            if v.len() == 2 {
                visited.push((pos, from.reverse()));
            }
            v
        }
        '-' => {
            let v = from.split_minus();
            if v.len() == 2 {
                visited.push((pos, from.reverse()));
            }
            v
        }
        a @ _ => panic!("unknown char: {a}"),
    };

    for nd in next_dirs {
        let next = con.next(pos, &nd);
        step(next, nd, &con, visited);
    }

    visited.push((pos, from));
}

pub fn unique_visited(visited: &Vec<(Coord, Direction)>) -> usize {
    let mut unique = Vec::new();
    for (c, _d) in visited {
        if !unique.contains(&c) {
            unique.push(c);
        }
    }

    unique.len()
}

pub fn part1(content: &str) -> usize {
    let contraption = Contraption::parse(content);
    let pos = (0, 0);
    let direction = Direction::Left;
    let mut visited = Vec::new();
    step(pos, direction, &contraption, &mut visited);
    unique_visited(&visited)
}

pub fn part2(content: &str) -> usize {
    let contraption = Contraption::parse(content);

    let direction = Direction::Above;
    let mut max_energized = contraption
        .top()
        .par_iter()
        .map(|c| {
            let mut visited = Vec::new();
            step(c.clone(), direction, &contraption, &mut visited);
            unique_visited(&visited)
        })
        .max()
        .unwrap();

    let direction = Direction::Below;
    max_energized = max_energized.max(
        contraption
            .bottom()
            .par_iter()
            .map(|c| {
                let mut visited = Vec::new();
                step(c.clone(), direction, &contraption, &mut visited);
                unique_visited(&visited)
            })
            .max()
            .unwrap(),
    );

    let direction = Direction::Left;
    max_energized = max_energized.max(
        contraption
            .left()
            .par_iter()
            .map(|c| {
                let mut visited = Vec::new();
                step(c.clone(), direction, &contraption, &mut visited);
                unique_visited(&visited)
            })
            .max()
            .unwrap(),
    );

    let direction = Direction::Right;
    max_energized = max_energized.max(
        contraption
            .right()
            .par_iter()
            .map(|c| {
                let mut visited = Vec::new();
                step(c.clone(), direction, &contraption, &mut visited);
                unique_visited(&visited)
            })
            .max()
            .unwrap(),
    );

    max_energized
}

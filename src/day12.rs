use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Record {
    literal: String,
    damaged: Vec<usize>,
}

pub fn permute(record: &Record, cache: Rc<RefCell<HashMap<Record, usize>>>) -> usize {
    if record.literal.is_empty() {
        if record.damaged.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if record.damaged.is_empty() {
        if record.literal.chars().any(|c| c == '#') {
            return 0;
        } else {
            return 1;
        }
    }

    if !record.is_literal_long_enough() {
        return 0;
    }

    if cache.borrow().contains_key(record) {
        return *cache.borrow().get(record).unwrap();
    }

    let result = match record.literal.chars().next().unwrap() {
        '.' => {
            let new_record = Record::new(&record.literal[1..], &record.damaged);
            permute(&new_record, cache.clone())
        }
        '#' => {
            let mut damaged = record.damaged.iter();
            let next = damaged.next().unwrap();

            let literal_chunk = &record.literal[0..*next];

            if literal_chunk.chars().any(|c| c == '.') {
                return 0;
            }

            if let Some(c) = record.literal.chars().nth(*next) {
                if c == '#' {
                    return 0;
                }
            };

            let new_damaged = damaged.cloned().collect();
            let new_literal = if record.literal.len() == *next {
                String::new()
            } else {
                record.literal[next + 1..].to_string()
            };
            let new_record = Record::new(&new_literal, &new_damaged);
            permute(&new_record, cache.clone())
        }
        '?' => ['.', '#']
            .iter()
            .map(|c| {
                let new_literal = record.literal.clone().replacen("?", &c.to_string(), 1);
                let new_record = Record::new(&new_literal, &record.damaged);
                permute(&new_record, cache.clone())
            })
            .sum(),
        _ => panic!("unknown char"),
    };

    cache.borrow_mut().insert(record.clone(), result);

    result
}

impl Record {
    pub fn new(literal: &str, damaged: &Vec<usize>) -> Self {
        Self {
            literal: literal.to_string(),
            damaged: damaged.clone(),
        }
    }

    pub fn parse(line: &str) -> Self {
        let (literal, damaged_str) = line.split_once(' ').unwrap();
        let damaged = damaged_str
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        Self {
            literal: literal.to_string(),
            damaged,
        }
    }

    pub fn parse_repeat(line: &str) -> Self {
        let (literal, damaged_str) = line.split_once(' ').unwrap();
        let literal = [literal; 5].join("?");

        let damaged = [damaged_str; 5]
            .join(",")
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Self { literal, damaged }
    }

    fn is_literal_long_enough(&self) -> bool {
        let lit_len = self.literal.len();
        let commas = self.damaged.len() - 1;
        let sum_d = self.damaged.iter().sum::<usize>();

        lit_len >= (commas + sum_d)
    }
}

pub fn part1(content: &str) -> usize {
    let lines = content.lines().collect::<Vec<_>>();

    let cache = Rc::new(RefCell::new(HashMap::new()));

    lines
        .iter()
        .map(|line| {
            let record = Record::parse(line);
            let p = permute(&record, cache.clone());
            p
        })
        .sum::<usize>()
}

pub fn part2(content: &str) -> usize {
    let lines = content.lines().collect::<Vec<_>>();

    let cache = Rc::new(RefCell::new(HashMap::new()));

    let count = lines
        .iter()
        .map(|line| {
            let record = Record::parse_repeat(line);
            permute(&record, cache.clone())
        })
        .sum::<usize>();

    count
}

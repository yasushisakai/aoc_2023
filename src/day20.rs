use std::collections::{HashMap, VecDeque};

use crate::day8::lcm;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Pulse {
    H,
    L,
}

#[derive(Debug)]
struct FlipFlop {
    is_on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { is_on: false }
    }

    fn input(&mut self, p: &Pulse) -> Option<Pulse> {
        if p == &Pulse::H {
            None
        } else {
            self.is_on = !self.is_on;
            if self.is_on {
                Some(Pulse::H)
            } else {
                Some(Pulse::L)
            }
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            inputs: HashMap::new(),
        }
    }

    fn input(&mut self, p: &Pulse, input_module: &str) -> Option<Pulse> {
        self.inputs.insert(input_module.to_string(), p.clone());
        if self.inputs.iter().all(|(_, p)| p == &Pulse::H) {
            Some(Pulse::L)
        } else {
            Some(Pulse::H)
        }
    }

    fn register(&mut self, input_module: &str) {
        self.inputs.insert(input_module.to_string(), Pulse::L);
    }
}

fn parse(
    content: &str,
) -> (
    HashMap<&str, Vec<&str>>,
    HashMap<&str, FlipFlop>,
    HashMap<&str, Conjunction>,
) {
    let mut network = HashMap::new();
    network.insert("button", vec!["broadcaster"]);
    let mut ffs = HashMap::new();
    let mut cons = HashMap::new();

    for line in content.lines() {
        let (source, desitnations) = line.split_once(" -> ").unwrap();

        let destinations: Vec<_> = desitnations.split(',').map(|d| d.trim()).collect();

        match source.split_at(1) {
            ("%", name) => {
                ffs.insert(name, FlipFlop::new());
                network.insert(name, destinations);
            }
            ("&", name) => {
                cons.insert(name, Conjunction::new());
                network.insert(name, destinations);
            }
            ("b", _) => {
                network.insert(source, destinations);
            }
            _ => panic!(),
        }
    }

    for (c_name, con) in cons.iter_mut() {
        for (s, ds) in network.iter() {
            if ds.iter().any(|n| n == c_name) {
                con.register(s);
            }
        }
    }

    (network, ffs, cons)
}

pub fn part1(content: &str) -> usize {
    let (network, mut ffs, mut cons) = parse(&content);
    let mut low = 0;
    let mut high = 0;

    for _i in 0..1000 {
        let mut q = VecDeque::new();
        q.push_back(("button", "broadcaster", Pulse::L));

        while let Some((source, dest, pulse)) = q.pop_front() {
            match pulse {
                Pulse::L => low += 1,
                Pulse::H => high += 1,
            }

            let Some(next_destinations) = network.get(dest) else {
                continue;
            };

            if ffs.contains_key(dest) {
                let ff = ffs.get_mut(dest).unwrap();
                if let Some(p) = ff.input(&pulse) {
                    for nd in next_destinations {
                        q.push_back((dest, nd, p.clone()));
                    }
                }
                continue;
            }

            if cons.contains_key(dest) {
                let con = cons.get_mut(dest).unwrap();
                let np = con.input(&pulse, source).unwrap();
                for nd in next_destinations {
                    q.push_back((dest, nd, np.clone()));
                }
                continue;
            }

            if source == "button" || source == "broadcaster" {
                for nd in next_destinations {
                    q.push_back((dest, nd, pulse.clone()));
                }
            }
        }
    }

    low * high
}

pub fn part2(content: &str) -> usize {
    let (network, mut ffs, mut cons) = parse(&content);

    let mut first_child = "rx";
    let mut second_children = Vec::new();
    for (s, ds) in network.iter() {
        if ds.contains(&"rx") {
            first_child = s;
            for (ss, sds) in network.iter() {
                if sds.contains(&first_child) {
                    second_children.push(*ss);
                }
            }
        }
    }

    let mut intervals: HashMap<&str, Option<usize>> =
        second_children.iter().map(|sc| (*sc, None)).collect();

    for i in 0usize.. {
        let mut q = VecDeque::new();
        q.push_back(("button", "broadcaster", Pulse::L));

        while let Some((source, dest, pulse)) = q.pop_front() {
            if second_children.contains(&source) && dest == first_child && pulse == Pulse::H {
                if intervals.get(source) == Some(&None) {
                    intervals.insert(source, Some(i + 1));
                }
            }

            if intervals.values().all(|v| v.is_some()) {
                return intervals
                    .values()
                    .map(|v| v.unwrap())
                    .fold(1, |acc, v| lcm(acc, v));
            }

            let Some(next_destinations) = network.get(dest) else {
                continue;
            };

            if ffs.contains_key(dest) {
                let ff = ffs.get_mut(dest).unwrap();
                if let Some(p) = ff.input(&pulse) {
                    for nd in next_destinations {
                        q.push_back((dest, nd, p.clone()));
                    }
                }
                continue;
            }

            if cons.contains_key(dest) {
                let con = cons.get_mut(dest).unwrap();
                let np = con.input(&pulse, source).unwrap();
                for nd in next_destinations {
                    q.push_back((dest, nd, np.clone()));
                }
                continue;
            }

            if source == "button" || source == "broadcaster" {
                for nd in next_destinations {
                    q.push_back((dest, nd, pulse.clone()));
                }
            }
        }
    }
    0
}

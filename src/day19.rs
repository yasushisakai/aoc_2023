use std::{collections::HashMap, panic};

type Workflows = HashMap<String, Vec<Step>>;

enum Step {
    Compare {
        spec: char,
        op: char,
        value: usize,
        target: String,
    },
    Goto(String),
}

impl Step {
    fn parse(s: &str) -> Self {
        match s.split_once(':') {
            Some((cond, target)) => {
                let target = target.to_string();
                let mut chars = cond.chars();
                let spec = chars.next().unwrap();
                let op = chars.next().unwrap();
                let value = chars.collect::<String>().parse().unwrap();
                Self::Compare {
                    spec,
                    op,
                    value,
                    target,
                }
            }
            None => Self::Goto(s.to_string()),
        }
    }
}

fn parse_workflow(line: &str) -> (String, Vec<Step>) {
    let (name, steps_str) = line.split_once('{').unwrap();
    let steps = steps_str
        .trim_end_matches('}')
        .split(',')
        .map(Step::parse)
        .collect();

    (name.to_string(), steps)
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn value(&self) -> usize {
        let s = self.x + self.m + self.a + self.s;
        s
    }

    fn get_value(&self, spec: &char) -> usize {
        match spec {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct RangePart {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl RangePart {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn spec_split_smaller(&self, spec: &char, value: &usize) -> (Self, Self) {
        let mut applied = self.clone();
        let mut except = self.clone();
        let v = value - 1;
        match spec {
            'x' => {
                applied.x.1 = applied.x.1.min(v);
                except.x.0 = except.x.0.max(*value);
            }
            'm' => {
                applied.m.1 = applied.m.1.min(v);
                except.m.0 = except.m.0.max(*value);
            }
            'a' => {
                applied.a.1 = applied.a.1.min(v);
                except.a.0 = except.a.0.max(*value);
            }
            's' => {
                applied.s.1 = applied.s.1.min(v);
                except.s.0 = except.s.0.max(*value);
            }
            _ => panic!(),
        }

        (applied, except)
    }

    fn spec_split_larger(&self, spec: &char, value: &usize) -> (Self, Self) {
        let mut applied = self.clone();
        let mut except = self.clone();
        let v = value + 1;
        match spec {
            'x' => {
                applied.x.0 = applied.x.0.max(v);
                except.x.1 = except.x.1.min(*value);
            }
            'm' => {
                applied.m.0 = applied.m.0.max(v);
                except.m.1 = except.m.1.min(*value);
            }
            'a' => {
                applied.a.0 = applied.a.0.max(v);
                except.a.1 = except.a.1.min(*value);
            }
            's' => {
                applied.s.0 = applied.s.0.max(v);
                except.s.1 = except.s.1.min(*value);
            }
            _ => panic!(),
        }

        (applied, except)
    }

    fn combinations(&self) -> usize {
        if self.x.1 < self.x.0 || self.m.1 < self.m.0 || self.a.1 < self.a.0 || self.s.1 < self.s.0
        {
            return 0;
        }

        let x = self.x.1 - self.x.0 + 1;
        let m = self.m.1 - self.m.0 + 1;
        let a = self.a.1 - self.a.0 + 1;
        let s = self.s.1 - self.s.0 + 1;

        x * m * a * s
    }
}

fn range_combinations(
    combination: RangePart,
    wf_name: &str,
    path: &Vec<String>,
    combinations: &mut Vec<RangePart>,
    workflows: &Workflows,
) {
    let mut path = path.clone();
    path.push(wf_name.to_string());

    if wf_name == "A" {
        combinations.push(combination);
        return;
    }

    if wf_name == "R" {
        return;
    }

    if let Some(wf) = workflows.get(wf_name) {
        let mut comb = combination;
        for step in wf {
            match step {
                Step::Goto(new_name) => {
                    range_combinations(comb.clone(), new_name, &path, combinations, workflows);
                }
                Step::Compare {
                    spec,
                    op,
                    value,
                    target,
                } => {
                    let (a, e) = match op {
                        '>' => comb.spec_split_larger(spec, value),
                        '<' => comb.spec_split_smaller(spec, value),
                        _ => panic!(),
                    };
                    range_combinations(a, target, &path, combinations, workflows);
                    comb = e;
                }
            }
        }
    }
}

fn process(part: &Part, workflows: &HashMap<String, Vec<Step>>) -> bool {
    let mut name = "in";

    while let Some(workflow) = workflows.get(name) {
        for step in workflow {
            let next = match step {
                Step::Goto(next) => Some(next),
                Step::Compare {
                    spec,
                    op,
                    value,
                    target,
                } => {
                    let part_value = part.get_value(spec);
                    let cond = match op {
                        '>' => &part_value > value,
                        '<' => &part_value < value,
                        _ => panic!(),
                    };
                    if cond {
                        Some(target)
                    } else {
                        None
                    }
                }
            };
            if let Some(n) = next {
                name = n;
                break;
            }
        }
    }

    name == "A"
}

pub fn part1(content: &str) -> usize {
    let (workflows, parts) = content.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Vec<Step>> = workflows.lines().map(parse_workflow).collect();

    let parts = parts
        .lines()
        .map(|s| {
            let mut specs = s.trim_matches(&['{', '}'][..]).split(',');
            let x = specs.next().unwrap()[2..].parse().unwrap();
            let m = specs.next().unwrap()[2..].parse().unwrap();
            let a = specs.next().unwrap()[2..].parse().unwrap();
            let s = specs.next().unwrap()[2..].parse().unwrap();
            Part { x, m, a, s }
        })
        .collect::<Vec<Part>>();

    let parts_accepted: Vec<&Part> = parts.iter().filter(|p| process(p, &workflows)).collect();

    parts_accepted.iter().map(|p| p.value()).sum()
}

pub fn part2(content: &str) -> usize {
    let (workflows, _parts) = content.split_once("\n\n").unwrap();
    let workflows: Workflows = workflows.lines().map(parse_workflow).collect();

    let mut combinations = Vec::new();

    range_combinations(
        RangePart::new(),
        "in",
        &Vec::new(),
        &mut combinations,
        &workflows,
    );

    combinations.iter().map(|r| r.combinations()).sum()
}

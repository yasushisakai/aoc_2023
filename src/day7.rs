use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    kind: HandKind,
    cards: Vec<u8>,
}

impl Hand {
    pub fn new(hand: &str) -> Self {
        let freq = hand.chars().fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|n| *n += 1).or_insert(1);
            map
        });

        let kind = match freq.len() {
            4 => HandKind::OnePair,
            3 => {
                if freq.iter().any(|(_, n)| *n == 3) {
                    HandKind::ThreeKind
                } else {
                    HandKind::TwoPair
                }
            }
            2 => {
                // full house or fourkind
                let (_, first) = freq.iter().next().unwrap();
                if first == &1 || first == &4 {
                    HandKind::FourKind
                } else {
                    HandKind::FullHouse
                }
            }
            1 => HandKind::FiveKind,
            _ => HandKind::HighCard,
        };

        let cards = hand
            .chars()
            .map(|c| match c {
                '2'..='9' => c.to_digit(10).unwrap() as u8,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            })
            .collect::<Vec<_>>();

        Self { kind, cards }
    }
    pub fn new_2(hand: &str) -> Self {
        let mut freq = hand.chars().fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|n| *n += 1).or_insert(1);
            map
        });

        let js = freq.remove(&'J').unwrap_or(0);

        let mut f_values = freq.values().cloned().collect::<Vec<u32>>();
        f_values.sort();
        f_values.reverse();

        let mut iter = f_values.iter();

        let max_freq = *iter.next().unwrap_or(&0);
        let next_freq = iter.next();

        let kind = match js + max_freq {
            5 => HandKind::FiveKind,
            4 => HandKind::FourKind,
            3 => {
                if next_freq == Some(&2) {
                    HandKind::FullHouse
                } else {
                    HandKind::ThreeKind
                }
            }
            2 => {
                if next_freq == Some(&2) {
                    HandKind::TwoPair
                } else {
                    HandKind::OnePair
                }
            }
            1 => HandKind::HighCard,
            _ => panic!(),
        };

        let cards = hand
            .chars()
            .map(|c| match c {
                'J' => 1,
                '2'..='9' => c.to_digit(10).unwrap() as u8,
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => 0,
            })
            .collect::<Vec<_>>();

        Self { kind, cards }
    }
}

pub fn part(content: &str, parse_fn: &dyn Fn(&str) -> Hand) -> u64 {
    let lines = content.lines();

    let mut bids: Vec<(Hand, u64)> = lines
        .map(|line| {
            let (hand_str, bid) = line.split_once(' ').unwrap();
            let hand = parse_fn(hand_str);
            let bid = bid.parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect();

    bids.sort_by(|(a, _), (b, _)| b.cmp(&a));
    bids.reverse();

    bids.iter().enumerate().fold(0u64, |acc, (i, line)| {
        let rank = i as u64 + 1;
        let bid = line.1;
        acc + rank * bid
    })
}

pub fn part1(content: &str) -> u64 {
    part(content, &Hand::new)
}

pub fn part2(content: &str) -> u64 {
    part(content, &Hand::new_2)
}

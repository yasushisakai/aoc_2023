use aoc_2023::day21::{part1, part2};
use std::fs;

fn main() {
    let day = 21;
    let path = format!("inputs/d{day}.txt");
    let content = fs::read_to_string(path).unwrap();
    let answer = part1(&content, 64);
    println!("part1: {answer}");

    let answer = part2(&content);
    println!("part2: {answer}");
}

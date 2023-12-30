use aoc_2023::day24;
use std::fs;

fn main() {
    let day = 24;
    let path = format!("inputs/d{day}.txt");
    let content = fs::read_to_string(path).unwrap();
    let answer = day24::part1(&content, 200000000000000.0, 400000000000000.0);
    println!("part1: {answer}");
    let answer = day24::part2(&content);
    println!("part2: {answer}");
}

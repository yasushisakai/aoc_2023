use aoc_2023::day14::part1;
use aoc_2023::day14::part2;
use std::fs;

fn main() {
    let day = 14;
    let path = format!("inputs/d{day}.txt");
    let content = fs::read_to_string(path).unwrap();
    let answer = part1(&content);
    println!("part1: {answer}");
    let answer = part2(&content);
    println!("part2: {answer}");
}

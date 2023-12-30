use aoc_2023::day22;
use std::fs;

fn main() {
    let day = 22;
    let path = format!("inputs/d{day}.txt");
    let content = fs::read_to_string(path).unwrap();
    let answer = day22::part1(&content);
    println!("part1: {answer}");
    let answer = day22::part2(&content);
    println!("part2: {answer}");
}

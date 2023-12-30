use std::fs;

use aoc_2023::day4;

fn main() {
    let day = 4;
    let path = format!("inputs/d{day}.txt");
    let content = fs::read_to_string(path).unwrap();
    let part1 = day4::part1(&content);
    println!("part1: {part1}");
    let part2 = day4::part2(&content);
    println!("part2: {part2}");
}

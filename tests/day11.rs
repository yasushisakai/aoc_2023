#[cfg(test)]
mod day11_tests {

    use aoc_2023::day11;

    const CONTENT: &'static str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    #[test]
    fn part1() {
        let answer = day11::part1(CONTENT);
        assert_eq!(answer, 374);
    }

    #[test]
    fn part2_1() {
        let answer = day11::part2(CONTENT, 10);
        assert_eq!(answer, 1030);
    }

    #[test]
    fn part2_2() {
        let answer = day11::part2(CONTENT, 100);
        assert_eq!(answer, 8410);
    }
}

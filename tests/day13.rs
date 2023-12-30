mod day13_tests {

    use aoc_2023::day13::{self, Direction};

    const CONTENT: &'static str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    const EXAMPLE: &'static str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;

    const TEST: &'static str = r#".####......####..
......####.......
#######..########
.#####.##.#####..
.......##.......#
##..###..###..###
.......##.......#"#;

    #[test]
    fn part1() {
        let answer = day13::part1(CONTENT);
        assert_eq!(answer, 405);
    }

    #[test]
    fn part1_1() {
        let pattern = day13::Pattern::parse(EXAMPLE);
        let line = pattern.find_offby(0);
        assert_eq!(line, (Direction::V, 5));
    }

    #[test]
    fn part1_2() {
        let pattern = day13::Pattern::parse(TEST);
        let line = pattern.find_offby(0);
        println!("{:?}", line);
        assert_eq!(line, (Direction::V, 8));
    }

    #[test]
    fn part2() {
        let answer = day13::part2(CONTENT);
        assert_eq!(answer, 400);
    }

    #[test]
    fn part2_1() {
        let pattern = day13::Pattern::parse(EXAMPLE);
        let line = pattern.find_offby(1);
        assert_eq!(line, (Direction::H, 3));
    }
}

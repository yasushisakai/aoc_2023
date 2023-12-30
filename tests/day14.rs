mod day14_tests {

    use aoc_2023::day14;

    const CONTENT: &'static str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    const CYCLE: &'static str = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#;

    const TWO_CYCLES: &'static str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#;

    const THREE_CYCLES: &'static str = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#;

    #[test]
    fn part1() {
        let answer = day14::part1(CONTENT);
        assert_eq!(answer, 136);
    }

    #[test]
    fn part2_cycles() {
        let content = CONTENT.lines().map(|l| l.to_string()).collect();
        let cycle: Vec<String> = CYCLE.lines().map(|l| l.to_string()).collect();
        let two_cycles: Vec<String> = TWO_CYCLES.lines().map(|l| l.to_string()).collect();
        let three_cycles: Vec<String> = THREE_CYCLES.lines().map(|l| l.to_string()).collect();
        let cycled = day14::cycle(content);
        assert_eq!(cycle, cycled);
        let cycled = day14::cycle(cycled);
        assert_eq!(two_cycles, cycled);
        let cycled = day14::cycle(cycled);
        assert_eq!(three_cycles, cycled);
    }

    #[test]
    fn part2() {
        let answer = day14::part2(CONTENT);
        assert_eq!(answer, 64);
    }
}

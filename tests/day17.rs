mod day17_tests {

    use aoc_2023::day17;

    const CONTENT: &'static str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

    const CONTENT2: &'static str = r#"111111111111
999999999991
999999999991
999999999991
999999999991"#;

    #[test]
    fn part1() {
        let answer = day17::part1(CONTENT);
        assert_eq!(answer, 102);
    }

    #[test]
    fn part2a() {
        let answer = day17::part2(CONTENT);
        assert_eq!(answer, 94);
    }

    #[test]
    fn part2b() {
        let answer = day17::part2(CONTENT2);
        assert_eq!(answer, 71);
    }
}

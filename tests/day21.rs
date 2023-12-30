mod day21_tests {

    use aoc_2023::day21;

    const CONTENT: &'static str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn part1() {
        let answer = day21::part1(CONTENT, 6);
        assert_eq!(answer, 16); // 64
    }

    #[test]
    fn part2a() {
        let answer = day21::part1(CONTENT, 10);
        assert_eq!(answer, 50);
        let answer = day21::part1(CONTENT, 50);
        assert_eq!(answer, 1594);
        let answer = day21::part1(CONTENT, 100);
        assert_eq!(answer, 6536);
        // let answer = day21::part1(CONTENT, 500);
        // assert_eq!(answer, 167004); // 64
        // let answer = day21::part1(CONTENT, 1000);
        // assert_eq!(answer, 668697); // 64
        // let answer = day21::part1(CONTENT, 5000);
        // assert_eq!(answer, 16733044); // 64
    }

    #[test]
    fn part2() {
        // a good representation
    }

    // #[test]
    // fn part2() {
    //     let answer = day21::part2(CONTENT);
    //     assert_eq!(answer, 0);
    // }
}

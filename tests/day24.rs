mod day24_tests {

    use aoc_2023::day24;

    const CONTENT: &'static str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    #[test]
    fn part1() {
        let answer = day24::part1(CONTENT, 7.0, 27.0);
        assert_eq!(answer, 2);
    }

    // #[test]
    // fn part2() {
    //     let answer = day24::part2(CONTENT);
    //     assert_eq!(answer, 0);
    // }
}

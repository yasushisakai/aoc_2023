mod day22_tests {

    use aoc_2023::day22;

    const CONTENT: &'static str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[test]
    fn part1() {
        let answer = day22::part1(CONTENT);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part2() {
        let answer = day22::part2(CONTENT);
        assert_eq!(answer, 7);
    }
}

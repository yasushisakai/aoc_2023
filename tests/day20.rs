mod day20_tests {

    use aoc_2023::day20;

    const CONTENT: &'static str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    const CONTENT2: &'static str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn part1a() {
        let answer = day20::part1(CONTENT);
        assert_eq!(answer, 32000000);
    }

    #[test]
    fn part1b() {
        let answer = day20::part1(CONTENT2);
        assert_eq!(answer, 11687500);
    }

    // #[test]
    // fn part2() {
    //     let answer = day20::part2(CONTENT);
    //     assert_eq!(answer, 0);
    // }
}

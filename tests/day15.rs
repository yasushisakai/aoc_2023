mod day15_tests {

    use aoc_2023::day15;

    const CONTENT: &'static str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn part1() {
        let answer = day15::part1(CONTENT);
        assert_eq!(answer, 1320);
    }

    #[test]
    fn part2() {
        let answer = day15::part2(CONTENT);
        assert_eq!(answer, 145);
    }
}

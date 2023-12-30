#[cfg(test)]
mod day8_tests {

    use aoc_2023::day8;

    const CONTENT: &'static str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn part1() {
        let answer = day8::part1(CONTENT);
        assert_eq!(answer, 6);
    }

    const CONTENT_PART: &'static str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn part2() {
        let answer = day8::part2(CONTENT_PART);
        assert_eq!(answer, 6);
    }
}

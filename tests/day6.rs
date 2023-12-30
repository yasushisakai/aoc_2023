#[cfg(test)]
mod day6_tests {

    use aoc_2023::day6;

    const CONTENT: &'static str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn part1() {
        let answer = day6::part1(CONTENT);
        assert_eq!(answer, 288);
    }

    #[test]
    fn part2() {
        let answer = day6::part2(CONTENT);
        assert_eq!(answer, 71503);
    }
}

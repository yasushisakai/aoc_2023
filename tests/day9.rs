#[cfg(test)]
mod day9_tests {

    use aoc_2023::day9;

    const CONTENT: &'static str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn part1() {
        let answer = day9::part1(CONTENT);
        assert_eq!(answer, 114);
    }

    #[test]
    fn part2() {
        let answer = day9::part2(CONTENT);
        assert_eq!(answer, 2);
    }
}

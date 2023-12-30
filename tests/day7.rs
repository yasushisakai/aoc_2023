#[cfg(test)]
mod day7_tests {

    use aoc_2023::day7::{self, Hand};

    const CONTENT: &'static str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn part1_cards_cmp() {
        let a = Hand::new("32T3K");
        let b = Hand::new("T55J5");
        assert!(b > a);
    }

    #[test]
    fn part1() {
        let answer = day7::part1(CONTENT);
        assert_eq!(answer, 6440);
    }

    #[test]
    fn part2() {
        let answer = day7::part2(CONTENT);
        assert_eq!(answer, 5905);
    }
}

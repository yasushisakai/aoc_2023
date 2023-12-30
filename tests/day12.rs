// #[cfg(test)]
mod day12_tests {

    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use aoc_2023::day12::{self, permute, Record};

    const CONTENT: &'static str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn part1() {
        let answer = day12::part1(CONTENT);
        assert_eq!(answer, 21);
    }

    #[test]
    fn part1_p1() {
        let cache = Rc::new(RefCell::new(HashMap::new()));
        let line = "#???#..??#? 5,4";
        let r = Record::parse(line);
        let p = permute(&r, cache.clone());
        assert_eq!(p, 1);
    }

    #[test]
    fn part2_parse() {
        let line = "???.### 1,1,3";
        let unwrapped = "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3";

        let r = Record::parse(unwrapped);
        let ru = Record::parse_repeat(line);

        assert_eq!(r, ru);
    }

    // #[test]
    // fn part2_p1() {
    //     let line = "???.### 1,1,3";
    //     let r = Record::parse_repeat(line);
    //     assert_eq!(r.count_ok(), 1);
    // }

    // #[test]
    // fn part2_p2() {
    //     let line = ".??..??...?##. 1,1,3";
    //     let r = Record::parse_repeat(line);
    //     assert_eq!(r.count_ok(), 16384);
    // }

    // #[test]
    // fn part2_p3() {
    //     let line = "?#?#?#?#?#?#?#? 1,3,1,6";
    //     let r = Record::parse_repeat(line);
    //     assert_eq!(r.count_ok(), 1);
    // }

    // #[test]
    // fn part2_p4() {
    //     let line = "????.#...#... 4,1,1";
    //     let r = Record::parse_repeat(line);
    //     assert_eq!(r.count_ok(), 16);
    // }

    // #[test]
    // fn part2_p5() {
    //     let line = "????.######..#####. 1,6,5";
    //     let r = Record::parse_repeat(line);
    //     assert_eq!(r.count_ok(), 2500);
    // }

    // #[test]
    // fn part2_p6() {
    //     let line = "?###???????? 3,2,1";
    //     let r = Record::parse_repeat(line);
    //     assert_eq!(r.count_ok(), 506250);
    // }

    #[test]
    fn part2() {
        let answer = day12::part2(CONTENT);
        assert_eq!(answer, 525152);
    }
}

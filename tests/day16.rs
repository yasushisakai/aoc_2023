mod day16_tests {

    use aoc_2023::day16::{self, step, unique_visited, Contraption, Direction};

    const CONTENT: &'static str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn part1() {
        let answer = day16::part1(CONTENT);
        assert_eq!(answer, 46);
    }

    #[test]
    fn part1_2() {
        let contraption = Contraption::parse(CONTENT);
        let pos = (3, 0);
        let direction = Direction::Above;
        let mut visited = Vec::new();
        step(pos, direction, &contraption, &mut visited);
        let e = unique_visited(&visited);
        assert_eq!(e, 51);
    }

    #[test]
    fn part2() {
        let answer = day16::part2(CONTENT);
        assert_eq!(answer, 51);
    }
}

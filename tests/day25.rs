mod day25_tests {

    use aoc_2023::day25;

    const CONTENT: &'static str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    #[test]
    fn part1() {
        let answer = day25::part1(CONTENT);
        assert_eq!(answer, 54);
    }

    // #[test]
    // fn part2() {
    //     let answer = day25::part2(CONTENT);
    //     assert_eq!(answer, 0);
    // }
}

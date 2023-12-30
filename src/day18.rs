trait Trench {
    fn area(self) -> i64;
}

impl<'a, T> Trench for T
where
    T: Iterator<Item = (&'a str, i64)>,
{
    fn area(self) -> i64 {
        let mut area = 0;
        let mut pos = (0, 0);
        let mut edge = 0;

        // shoelace formula
        for (d, v) in self {
            edge += v;
            match d {
                "R" => {
                    area += 2 * pos.1 * (-v);
                    pos.0 += v;
                }
                "L" => {
                    area += 2 * pos.1 * v;
                    pos.0 -= v;
                }
                "D" => {
                    pos.1 += v;
                }
                "U" => {
                    pos.1 -= v;
                }
                _ => panic!(),
            }
        }

        area /= 2;

        // Picks formula: A = i + b/2 - 1
        // i = A - b/2 + 1
        let inner = area - edge / 2 + 1;
        // we want i + b
        inner + edge
    }
}

pub fn part1(content: &str) -> i64 {
    let trench = content.lines().map(|l| {
        let mut s = l.split(' ');
        let d = s.next().unwrap();
        let v = s.next().unwrap().parse::<i64>().unwrap();
        (d, v)
    });

    trench.area()
}

pub fn part2(content: &str) -> i64 {
    let trench = content.lines().map(|l| {
        let (_, code) = l.split_once('#').unwrap();
        let v = i64::from_str_radix(&code[..5], 16).unwrap();
        let d = match code.chars().nth(5).unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => panic!(),
        };
        (d, v)
    });

    trench.area()
}

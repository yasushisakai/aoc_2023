use std::fs;

fn main() {
    let day = 2;
    let path = format!("inputs/d{day}.txt");

    let content = fs::read_to_string(path).unwrap();
    let mut sum = 0;

    for line in content.lines() {
        let (_, results) = line.split_once(':').unwrap();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for reveal in results.split(';') {
            for single_color in reveal.split(',') {
                let trimmed = single_color.trim();
                let (num, color) = trimmed.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                match color {
                    "red" => red = red.max(num),
                    "green" => green = green.max(num),
                    "blue" => blue = blue.max(num),
                    _ => panic!("not rgb"),
                };
            }
        }

        let power = red * green * blue;
        sum += power;
    }

    println!("{sum}");
}

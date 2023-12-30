use std::fs;

fn main() {
    let day = 1;
    let path = format!("inputs/d{day}.txt");

    let content = fs::read_to_string(path).unwrap();

    // the puzzle doesn't specify what to do with this: "oneight"
    // the fourth example shows that "xtwone3four" should be 24, but this
    // still doesn't answer "xtwone4twone" -> is this 21 or 22..
    // because the order of evaluation is not defined.
    let converted = content
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let mut sum = 0;

    for line in converted.lines() {
        let mut digits = line.chars().filter(|c| match c {
            '0'..='9' => true,
            _ => false,
        });

        let first = digits.nth(0).unwrap();
        let last = digits.last().unwrap_or(first);

        let number: u32 = format!("{first}{last}").parse::<u32>().unwrap();

        sum += number;
    }

    println!("{sum}");
}

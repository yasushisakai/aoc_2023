fn card_matching(line: &str) -> u8 {
    let (_, card) = line.split_once(':').unwrap();
    let numbers = card
        .split('|')
        .map(|ns| {
            ns.trim()
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let winning = &numbers[0];
    let mine = &numbers[1];

    let mut matching = 0;

    for m in mine {
        if winning.contains(&m) {
            matching += 1;
        }
    }
    matching
}

pub fn part1(content: &str) -> u32 {
    let mut answer = 0;

    for line in content.lines() {
        let hit = card_matching(line);

        if hit > 0 {
            let points = 2u32.pow((hit - 1) as u32);
            answer += points;
        }
    }

    answer
}

pub fn part2(content: &str) -> u32 {
    let lines: Vec<&str> = content.lines().collect();
    let mut card_numbers = vec![1u32; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let num = card_numbers[i];
        let matching = card_matching(line);

        for j in i + 1..(i + 1 + matching as usize) {
            card_numbers[j] += num;
        }
    }

    card_numbers.iter().sum()
}

use std::vec;

fn get_numbers(line: &str) -> Vec<u32> {
    let (_, numbers) = line.split_once(':').unwrap();
    numbers
        .trim()
        .split(' ')
        .filter(|e| !e.is_empty())
        .map(|e| e.parse::<u32>().unwrap())
        .collect()
}

fn get_number(line: &str) -> f64 {
    let (_, number) = line.split_once(':').unwrap();

    number
        .trim()
        .chars()
        .filter(|c| !(*c == ' '))
        .collect::<String>()
        .parse::<f64>()
        .unwrap()
}

pub fn part2(content: &str) -> u64 {
    let mut lines = content.lines();
    let time = get_number(lines.next().unwrap());
    let distance = get_number(lines.next().unwrap());

    let time_sq = (time as u64).pow(2) as f64;

    let min = ((time - f64::sqrt(time_sq - 4.0 * distance)) / 2.0 + 0.00001).ceil() as u64;

    let max = ((time + f64::sqrt(time_sq - 4.0 * distance)) / 2.0).ceil() as u64;

    println!("{min} -> {max}");

    max - min
}

pub fn part1(content: &str) -> u32 {
    let mut lines = content.lines();

    let times = get_numbers(lines.next().unwrap());
    let distances = get_numbers(lines.next().unwrap());

    let mut ways_to_beat = vec![0u32; times.len()];

    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];

        let min = ((*time as f32 - f32::sqrt((time.pow(2) - 4 * distance) as f32)) / 2.0 + 0.00001)
            .ceil() as u32;

        let max =
            ((*time as f32 + f32::sqrt((time.pow(2) - 4 * distance) as f32)) / 2.0).ceil() as u32;

        let ways = (max - min) as u32;

        // let mut start = 0;
        // let mut end = *time;

        // for s in 0..=*time {
        //     let remaining = time - s;
        //     let dist = remaining * s;
        //     if dist > distance {
        //         start = s;
        //         break;
        //     }
        // }

        // for e in start..=*time {
        //     let remaining = time - e;
        //     let dist = remaining * e;
        //     if dist <= distance {
        //         end = e;
        //         break;
        //     }
        // }

        // let ways = end - start;
        ways_to_beat[i] = ways;
    }

    ways_to_beat.iter().product()
}

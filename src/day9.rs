pub fn part1(content: &str) -> i32 {
    let lines = content.lines();

    let predictions: Vec<i32> = lines
        .map(|line| {
            let numbers: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

            let mut last_values = Vec::new();
            let mut current_diff = numbers;

            loop {
                last_values.push(*current_diff.iter().last().unwrap());
                let next_diff: Vec<i32> = current_diff.windows(2).map(|s| s[1] - s[0]).collect();

                if next_diff.len() == 1 {
                    last_values.push(*next_diff.iter().last().unwrap());
                    break;
                }

                if match &next_diff[..] {
                    [first, rest @ ..] => rest.iter().all(|x| x == first),
                    _ => panic!(),
                } {
                    last_values.push(*next_diff.iter().last().unwrap());
                    break;
                }

                current_diff = next_diff;
            }
            last_values.iter().sum()
        })
        .collect();

    predictions.iter().sum()
}

pub fn part2(content: &str) -> i32 {
    let lines = content.lines();

    let predictions: Vec<i32> = lines
        .map(|line| {
            let numbers: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

            let mut first_values = Vec::new();
            let mut current_diff = numbers;

            loop {
                first_values.push(*current_diff.iter().next().unwrap());
                let next_diff: Vec<i32> = current_diff.windows(2).map(|s| s[1] - s[0]).collect();

                if next_diff.len() == 1 {
                    first_values.push(*next_diff.iter().next().unwrap());
                    break;
                }

                if match &next_diff[..] {
                    [first, rest @ ..] => rest.iter().all(|x| x == first),
                    _ => panic!(),
                } {
                    first_values.push(*next_diff.iter().next().unwrap());
                    break;
                }

                current_diff = next_diff;
            }

            first_values.reverse();

            let mut diff = first_values.iter();
            let delta = *diff.next().unwrap();

            diff.fold(delta, |acc, dl| *dl - acc)
        })
        .collect();

    predictions.iter().sum()
}

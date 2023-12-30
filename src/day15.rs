fn hash(s: &str) -> usize {
    s.chars()
        .filter(|c| c != &'\n')
        .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

pub fn part1(content: &str) -> usize {
    let strings = content.split(',');

    strings.fold(0, |acc, s| {
        let value = hash(s);
        acc + value
    })
}

pub fn part2(content: &str) -> usize {
    let mut hash_map: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    let operations = content.split(',');

    for o in operations {
        let o = o.trim();
        let (new_label, new_num) = o.split_once(&['=', '-'][..]).unwrap();

        let box_id = hash(new_label);
        let elements = hash_map.get_mut(box_id).unwrap();

        if new_num.is_empty() {
            // minus
            if let Some(i) = elements.iter().position(|(l, _)| l == &new_label) {
                elements.remove(i);
            }
        } else {
            // update
            let n = new_num.parse::<usize>().unwrap();
            if let Some(i) = elements.iter().position(|(l, _)| l == &new_label) {
                let elem = elements.get_mut(i).unwrap();
                *elem = (new_label, n);
            } else {
                elements.push((new_label, n));
            }
        }
    }

    // calculate focus_power
    hash_map
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let box_num = i + 1;
            b.iter().enumerate().fold(0, |acc, (j, (_, f_length))| {
                acc + box_num * (j + 1) * f_length
            })
        })
        .sum()
}

use std::fs;

type NumbersVec = Vec<(u32, usize, usize, usize)>;
type SymbolsVec = Vec<(usize, usize)>;

fn make_num(num_vec: &mut Vec<char>, start: (usize, usize), numbers: &mut NumbersVec) {
    if !num_vec.is_empty() {
        let num = num_vec.iter().collect::<String>().parse::<u32>().unwrap();
        numbers.push((num, start.0, start.1, num_vec.len()));
        num_vec.clear();
    }
}

fn main() {
    let day = 3;
    let path = format!("inputs/d{day}.txt");

    //     let content = r#"467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598.."#;

    let content = fs::read_to_string(path).unwrap();

    let mut numbers: NumbersVec = vec![];
    let mut symbols: SymbolsVec = vec![];

    for (y, line) in content.lines().enumerate() {
        let mut num_vec = vec![];
        let mut num_start: (usize, usize) = (0, 0);
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    if num_vec.is_empty() {
                        num_start = (x, y);
                    }
                    num_vec.push(c);
                }

                '*' => {
                    make_num(&mut num_vec, num_start, &mut numbers);
                    symbols.push((x, y));
                }

                _ => make_num(&mut num_vec, num_start, &mut numbers),
            }
        }
        make_num(&mut num_vec, num_start, &mut numbers);
    }

    let mut sum = 0;
    // filters symbols to find gears
    for (x, y) in symbols {
        let mut adjacents = vec![];

        for (num, sx, sy, l) in numbers.iter() {
            let starty = ((*sy as i32) - 1).max(0) as usize;
            let endy = sy + 1;
            let startx = ((*sx as i32) - 1).max(0) as usize;
            let endx = sx + l;

            if x >= startx && x <= endx && y >= starty && y <= endy {
                adjacents.push(num);
            }
        }
        if adjacents.len() == 2 {
            sum += adjacents[0] * adjacents[1];
        }
    }

    println!("{sum}");
}

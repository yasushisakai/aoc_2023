fn expand_location(
    coordinates: &(usize, usize),
    expand_x: &Vec<usize>,
    expand_y: &Vec<usize>,
    multiply: usize,
) -> (usize, usize) {
    let (ex, ey) = expand_factor(coordinates, expand_x, expand_y);
    let (x, y) = coordinates;
    // this is really like
    let nx = (*x - ex) + ex * multiply;
    let ny = (*y - ey) + ey * multiply;

    (nx, ny)
}

fn expand_factor(
    coordinates: &(usize, usize),
    expand_x: &Vec<usize>,
    expand_y: &Vec<usize>,
) -> (usize, usize) {
    let (x, y) = coordinates;
    let exp_x = expand_x.iter().filter(|ex| *ex < &x).count();
    let exp_y = expand_y.iter().filter(|ey| *ey < &y).count();

    (exp_x, exp_y)
}

pub fn part1(content: &str) -> usize {
    let lines = content.lines();

    let x_max = lines.clone().next().unwrap().chars().count();

    // let mut expand_x = vec![];
    let mut expand_y = vec![];
    let mut galaxies_x = vec![];
    let mut galaxies = vec![];

    for (y, line) in lines.enumerate() {
        let mut does_y_expand = true;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                does_y_expand = false;
                galaxies_x.push(x);
                galaxies.push((x, y));
            }
        }
        if does_y_expand {
            expand_y.push(y);
        }
    }

    let expand_x: Vec<usize> = (0..x_max)
        .filter(|x| !galaxies_x.iter().any(|gx| gx == x))
        .collect();

    let galaxies: Vec<_> = galaxies
        .iter()
        .map(|g| expand_location(g, &expand_x, &expand_y, 2))
        .collect();

    let mut total = 0;

    for (i, g) in galaxies.iter().enumerate() {
        for j in (i + 1)..(galaxies.len()) {
            let other = galaxies[j];
            let dx = g.0.abs_diff(other.0);
            let dy = g.1.abs_diff(other.1);
            let dist = dx + dy;
            total += dist;
        }
    }

    total
}

pub fn part2(content: &str, multiply: usize) -> usize {
    let lines = content.lines();

    let x_max = lines.clone().next().unwrap().chars().count();

    let mut expand_y = vec![];
    let mut galaxies_x = vec![];
    let mut galaxies = vec![];

    for (y, line) in lines.enumerate() {
        let mut does_y_expand = true;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                does_y_expand = false;
                galaxies_x.push(x);
                galaxies.push((x, y));
            }
        }
        if does_y_expand {
            expand_y.push(y);
        }
    }

    let expand_x: Vec<usize> = (0..x_max)
        .filter(|x| !galaxies_x.iter().any(|gx| gx == x))
        .collect();

    let galaxy_factors: Vec<_> = galaxies
        .iter()
        .map(|g| expand_factor(g, &expand_x, &expand_y))
        .collect();

    let mut total = (0, 0);

    for i in 0..(galaxies.len()) {
        for j in (i + 1)..(galaxies.len()) {
            // we calculate the difference separately
            let a = galaxies[i];
            let af = galaxy_factors[i];
            let b = galaxies[j];
            let bf = galaxy_factors[j];

            let coord = (a.0 - af.0).abs_diff(b.0 - bf.0) + (a.1 - af.1).abs_diff(b.1 - bf.1);

            let factor = af.0.abs_diff(bf.0) + af.1.abs_diff(bf.1);

            total = (total.0 + coord, total.1 + factor);
        }
    }

    total.0 + total.1 * multiply
}

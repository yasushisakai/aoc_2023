use crate::day8::gcd;
use itertools::Itertools;

pub fn part1(content: &str, min: f64, max: f64) -> usize {
    let hailstones: Vec<(f64, f64, f64, f64)> = content
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").unwrap();
            let pos: Vec<f64> = p
                .split(", ")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect();

            let vel: Vec<f64> = v
                .split(", ")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect();

            // a = vy / vx;
            let a = vel[1] / vel[0];
            // py = a * px + b;
            // b = py - a * px;
            let b = pos[1] - a * pos[0];
            (a, b, pos[0], vel[0])
        })
        .collect();

    let mut count = 0;

    for (i, (a, b, apx, avx)) in hailstones.iter().enumerate() {
        for j in (i + 1)..hailstones.len() {
            let (c, d, bpx, bvx) = hailstones.get(j).unwrap();

            if a == c {
                continue;
            }

            // intersection
            let ix = (d - b) / (a - c);
            let iy = ((a * d) - (b * c)) / (a - c);

            if ix < min || ix > max || iy < min || iy > max {
                continue;
            }

            // x = vx * t + px;
            // t = (x - px) / vx;
            let ta = (ix - apx) / avx;

            if ta < 0.0 {
                continue;
            }

            let tb = (ix - bpx) / bvx;

            if tb < 0.0 {
                continue;
            }

            count += 1;
        }
    }

    count
}

fn find_common_velocity(common: &Vec<(isize, isize, usize, usize)>) -> isize {
    for i in -300.. {
        let mut is_v = true;
        for (pv, diff, _, _) in common {
            let factor = pv + i;
            if factor == 0 {
                is_v = false;
                break;
            }
            if diff % factor != 0 {
                is_v = false;
                break;
            }
        }
        if is_v {
            return -i;
        }
    }

    panic!("should not happen");
}

pub fn part2(content: &str) -> isize {
    let hailstones: Vec<(isize, isize, isize, isize, isize, isize)> = content
        .lines()
        .map(|line| {
            let (poss, vels) = line.split_once(" @ ").unwrap();
            let pos: Vec<isize> = poss
                .split(", ")
                .map(|n| n.parse::<isize>().unwrap())
                .collect();

            let vel: Vec<isize> = vels
                .split(", ")
                .map(|n| n.parse::<isize>().unwrap())
                .collect();

            (pos[0], pos[1], pos[2], vel[0], vel[1], vel[2])
        })
        .collect();

    let mut common_vx = Vec::new();
    let mut common_vy = Vec::new();
    let mut common_vz = Vec::new();

    for (i, ha) in hailstones.iter().enumerate() {
        for j in i + 1..hailstones.len() {
            let hb = hailstones.get(j).unwrap();
            if ha.3 == hb.3 {
                // vx are the same
                common_vx.push((ha.3, ha.0 - hb.0, i, j));
            }
            if ha.4 == hb.4 {
                // vy are the same
                common_vy.push((ha.4, ha.1 - hb.1, i, j));
            }
            if ha.5 == hb.5 {
                // vz are the same
                common_vz.push((ha.5, ha.2 - hb.2, i, j));
            }
        }
    }

    common_vx.sort_by(|a, b| a.0.abs().cmp(&b.0.abs()));
    common_vy.sort_by(|a, b| a.0.abs().cmp(&b.0.abs()));
    common_vz.sort_by(|a, b| a.0.abs().cmp(&b.0.abs()));

    let vx = find_common_velocity(&common_vx);
    let vy = find_common_velocity(&common_vy);
    let vz = find_common_velocity(&common_vz);

    // take two hailstones that has common vx;
    let (cv, diff, i, j) = common_vx[0];
    let (pix, piy, piz, vix, viy, viz) = hailstones.get(i).unwrap();
    let (_, pjy, _, _, vjy, _) = hailstones.get(j).unwrap();

    let epoch = diff / (cv - vx);

    // we change axis and use y since, x will cancel out;
    let dist = epoch * vy;
    // (pjy + (n + e) * vjy) - (piy + n * viy) = dist
    // pjy + n * vjy + e * vjy - piy - n * viy = dist
    // n * (vjy - viy) + pjy + e * vjy - piy = dist
    // n * (vjy - viy) = (dist - pjy - e * vjy + piy)
    let n = (dist - pjy - epoch * vjy + piy) / (vjy - viy);

    let px = (n * vix + pix) - n * vx;
    let py = (n * viy + piy) - n * vy;
    let pz = (n * viz + piz) - n * vz;

    px + py + pz
}

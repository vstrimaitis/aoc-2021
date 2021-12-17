use regex::Regex;
use std::cmp::{max, min};
use std::iter::repeat;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let target: Vec<i32> = re
        .find_iter(input)
        .map(|m| m.as_str().parse().expect("Failed to parse input"))
        .collect();
    assert_eq!(target.len(), 4);
    let x1 = target[0];
    let x2 = target[1];
    let y1 = target[2];
    let y2 = target[3];

    let ans1 = (-y1) * (-y1 - 1) / 2;
    let ans2 = solve2_math(x1, x2, y1, y2);

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn solve2_math(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let calc_time = |v_0: i32, c_t: i32| -> f64 {
        let d = (1.0 + 2.0 * v_0 as f64).powf(2.0) - 8.0 * c_t as f64;
        if d < 0.0 {
            return 1000000.0;
        }
        let sqrt_d = d.sqrt();
        let t1 = (1.0 + 2.0 * v_0 as f64 - sqrt_d) / 2.0;
        let t2 = (1.0 + 2.0 * v_0 as f64 + sqrt_d) / 2.0;
        if t1 >= 0.0 {
            return t1;
        }
        if t2 >= 0.0 {
            return t2;
        }
        1000000.0
    };

    let calc_coord = |c_0: i32, v_0: i32, t: i32, v_lower_bound: i32| -> i32 {
        let v_t = max(v_lower_bound, v_0 - t + 1);
        let delta = v_0 * (v_0 + 1) / 2 - (v_t - 1) * v_t / 2;
        c_0 + delta
    };

    let hits_target = |vx: i32, vy: i32| -> bool {
        let tx1 = calc_time(vx, x1).ceil() as i32;
        let tx2 = calc_time(vx, x2).floor() as i32;
        let ty1 = calc_time(vy, y2).ceil() as i32;
        let ty2 = calc_time(vy, y1).floor() as i32;
        let t1 = max(tx1, ty1);
        let t2 = min(tx2, ty2);
        for t in t1..t2 + 1 {
            let x = calc_coord(0, vx, t, 0);
            let y = calc_coord(0, vy, t, i32::MIN);
            if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                return true;
            }
        }
        false
    };

    let vx_min = ((((1 + 8 * x1) as f64).sqrt() - 1.0) / 2.0).ceil() as i32;
    let vx_max = x2;
    let mut ans = 0;
    for vx in vx_min..vx_max + 1 {
        let tx1 = calc_time(vx, x1).ceil();
        let tx2 = calc_time(vx, x2).floor();

        let vy1 = ((y1 as f64 + tx1 * (tx1 - 1.0) / 2.0) / tx1).floor() as i32;
        let vy2 = ((y2 as f64 + tx2 * (tx2 - 1.0) / 2.0) / tx2).ceil() as i32;

        let vy_min = max(y1, vy1);
        let vy_max = min(-y1 - 1, vy2);

        for vy in vy_min..vy_max + 1 {
            if hits_target(vx, vy) {
                ans += 1;
            }
        }
    }
    ans
}

#[allow(dead_code)]
fn solve2_brute(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let hits_target = |(vx, vy)| -> bool {
        let (mut x, mut y) = (0, 0);
        let (mut vx, mut vy) = (vx, vy);
        while x <= x2 && y >= y1 {
            x += vx;
            y += vy;
            if vx > 0 {
                vx -= 1;
            }
            vy -= 1;
            if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
                return true;
            }
        }
        false
    };
    (1..x2 + 1)
        .flat_map(|vx| repeat(vx).zip(y1..-y1 + 1))
        .filter(|&p| hits_target(p))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/17.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "target area: x=20..30, y=-10..-5".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("45").as_deref());
        assert_eq!(p2.as_deref(), Some("112").as_deref());
    }
}

use crate::common::*;
use regex::Regex;
use std::iter;

type Line = (i32, i32, i32, i32);

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let re = Regex::new(r"[^\d]+").unwrap();
    let mut lines: Vec<Line> = get_nonempty_lines(input)
        .map(|l| {
            re.split(l)
                .map(|x| x.parse::<i32>().expect("Failed to parse coordinate"))
                .collect()
        })
        .map(to_line)
        .collect();

    // put the axial lines in front - then we can process all lines
    // in a single sweep
    lines.sort_by(|a, b| is_axial(b).cmp(&is_axial(a)));
    let (ans1, ans2) = count_intersections_combined(&lines);

    // let ans1 = count_intersections(&lines, is_axial);
    // let ans2 = count_intersections(&lines, |_| true);

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn count_intersections_combined(lines: &Vec<Line>) -> (u16, u16) {
    let mut counts = vec![0u8; 1000 * 1000];

    let mut ans1 = 0;
    let mut ans2 = 0;
    lines.iter().for_each(|&l| {
        let (x, y, xx, yy) = l;
        let gen_sequence = |from: i32, to: i32| {
            iter::successors(Some(from), move |x| Some(x + (to - from).signum()))
        };
        gen_sequence(x, xx)
            .zip(gen_sequence(y, yy))
            .take((x - xx).abs().max((y - yy).abs()) as usize + 1 as usize)
            .for_each(|(x, y)| {
                let id = (1000 * x + y) as usize;
                if counts[id] == 1 {
                    if is_axial(&l) {
                        ans1 += 1;
                    }
                    ans2 += 1;
                }
                counts[id] += 1;
            });
    });

    (ans1, ans2)
}

// fn count_intersections(lines: &Vec<Line>, line_pred: fn(&Line) -> bool) -> i32 {
//     let max_coord = lines
//         .iter()
//         .flat_map(|&(a, b, c, d)| vec![a, b, c, d])
//         .max()
//         .unwrap() as usize;
//     let mut counts = vec![vec![0; max_coord+1]; max_coord+1];

//     let mut ans = 0;
//     for l in lines {
//         if !line_pred(l) {
//             continue;
//         }
//         let &(x1, y1, x2, y2) = l;
//         let dx = (x2-x1).signum();
//         let dy = (y2-y1).signum();
//         let mut x = x1;
//         let mut y = y1;
//         while !(x == x2+dx && y == y2+dy) {
//             counts[x as usize][y as usize] += 1;
//             if counts[x as usize][y as usize] == 2 {
//                 ans += 1;
//             }
//             x += dx;
//             y += dy;
//         }
//     }
//     ans
// }

fn is_axial(line: &Line) -> bool {
    let (x1, y1, x2, y2) = line;
    x1 == x2 || y1 == y2
}

fn to_line(v: Vec<i32>) -> Line {
    assert!(v.len() == 4);
    (v[0], v[1], v[2], v[3])
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/05.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("5").as_deref());
        assert_eq!(p2.as_deref(), Some("12").as_deref());
    }
}

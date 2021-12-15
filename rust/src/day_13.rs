use crate::common::*;
use crate::ocr;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let (input_points, input_splits) = input.split_once("\n\n").expect("Failed to parse input");
    let pts: Vec<(i32, i32)> = get_nonempty_lines(&input_points.to_string())
                .map(|l| l.split_once(',').expect("Failed to parse point"))
                .map(|(x, y)| (
                    x.parse::<i32>().expect("Failed to x coordinate"),
                    y.parse::<i32>().expect("Failed to parse y coordinate")
                ))
                .collect();
    let folds: Vec<(char, i32)> = get_nonempty_lines(&input_splits.to_string())
                .map(|l| l.trim_start_matches("fold along "))
                .map(|l| l.split_once('=').expect("Failed to parse split"))
                .map(|(axis, coord)| (
                    axis.chars().next().expect("Failed to parse split axis"),
                    coord.parse::<i32>().expect("Failed to parse split coordinate")
                ))
                .collect();

    let mut pts_after_first_fold = fold(&pts, folds[0]);
    pts_after_first_fold.sort();
    pts_after_first_fold.dedup();
    let ans1 = pts_after_first_fold.len();

    let folded_pts = folds.iter().fold(pts, |p, &f| fold(&p, f));
    let drawing = draw(&folded_pts);
    let ans2 = ocr::parse(&drawing);

    (Some(ans1.to_string()), Some(ans2))
}

fn draw(pts: &Vec<(i32, i32)>) -> String {
    let xs: Vec<i32> = pts.iter().map(|&(x, _)| x).collect();
    let ys: Vec<i32> = pts.iter().map(|&(_, y)| y).collect();
    let min_x = *xs.iter().min().unwrap();
    let max_x = *xs.iter().max().unwrap();
    let min_y = *ys.iter().min().unwrap();
    let max_y = *ys.iter().max().unwrap();
    

    (min_y..max_y+1).map(|i|
        (min_x..max_x+1)
            .map(|j|
                if pts.contains(&(j, i)) {
                    '#'
                } else {
                    '.'
                }
            )
            .collect::<String>()
        )
        .collect::<Vec<String>>()
        .join("\n")
}

fn fold(pts: &Vec<(i32, i32)>, (axis, fold_coord): (char, i32)) -> Vec<(i32, i32)> {
    match axis {
        'x' => fold_x(pts, fold_coord),
        'y' => fold_y(pts, fold_coord),
        _ => panic!("Invalid axis name")
    }
}

fn fold_x(pts: &Vec<(i32, i32)>, fold_x: i32) -> Vec<(i32, i32)> {
    pts.iter()
        .map(|&(x, y)| if x < fold_x { (x, y) } else { (2*fold_x - x, y)})
        .collect()
}

fn fold_y(pts: &Vec<(i32, i32)>, fold_y: i32) -> Vec<(i32, i32)> {
    pts.iter()
        .map(|&(x, y)| if y < fold_y { (x, y) } else { (x, 2*fold_y - y)})
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/13.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("17").as_deref());
        assert_eq!(p2.as_deref(), Some("□").as_deref());
    }
}

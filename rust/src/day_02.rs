use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let commands: Vec<(&str, i32)> = get_nonempty_lines(input)
        .map(|l| l.split_once(' ').expect("Failed to split line into parts"))
        .map(|(x, y)| (x, y.parse::<i32>().expect("Failed to parse amount")))
        .collect();
    let (x1, y1) = commands
        .iter()
        .fold((0, 0), |(x, y), &(op, delta)| if op == "down" {
            (x, y+delta)
        } else if op == "up" {
            (x, y-delta)
        } else {
            (x+delta, y)
        });
    let (x2, y2, _) = commands
        .iter()
        .fold((0, 0, 0), |(x, y, aim), &(op, delta)| if op == "down" {
            (x, y, aim+delta)
        } else if op == "up" {
            (x, y, aim-delta)
        } else {
            (x+delta, y+aim*delta, aim)
        });
    
    (Some((x1*y1).to_string()), Some((x2*y2).to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/02.txt");
        b.iter(|| black_box(solve(&input.to_string())));
    }

    #[test]
    fn sample() {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2
".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("150").as_deref());
        assert_eq!(p2.as_deref(), Some("900").as_deref());
    }
}

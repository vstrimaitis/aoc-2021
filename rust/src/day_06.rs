use std::collections::VecDeque;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let nums: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().expect("Failed to parse input"))
        .collect();
    let mut ans1 = 0;

    let mut counts = VecDeque::from(vec![0i64; 9]);
    for x in nums {
        counts[x as usize] += 1;
    }
    for day in 0..256 {
        if day == 80 {
            ans1 = counts.iter().sum();
        }
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    let ans2: i64 = counts.iter().sum();

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/06.txt");
        b.iter(|| black_box(solve(&input.to_string())));
    }

    #[test]
    fn sample() {
        let data = "3,4,3,1,2".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("5934").as_deref());
        assert_eq!(p2.as_deref(), Some("26984457539").as_deref());
    }
}

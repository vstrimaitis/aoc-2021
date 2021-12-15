use std::cmp::min;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let mut nums: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().expect("Failed to parse input"))
        .collect();
    
    let ans1 = solve1(&mut nums);
    let ans2 = solve2(&nums);

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn solve1(nums: &mut Vec<i32>) -> i32 {
    nums.sort();
    let p = nums[nums.len() / 2];
    calc_fuel_usage(&nums, p, |d| d)
}

fn solve2(nums: &Vec<i32>) -> i32 {
    let avg= nums.iter().sum::<i32>() as f32 / nums.len() as f32;
    let p1 = avg.floor() as i32;
    let p2 = avg.ceil() as i32;
    let cost_fn = |d| d*(d+1)/2;
    min(
        calc_fuel_usage(&nums, p1, cost_fn),
        calc_fuel_usage(&nums, p2, cost_fn),
    )
}

fn calc_fuel_usage(nums: &Vec<i32>, target: i32, cost_fn: fn(i32) -> i32) -> i32 {
    nums.iter()
        .map(|x| (x-target).abs())
        .map(cost_fn)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/07.txt");
        b.iter(|| black_box(solve(&input.to_string())));
    }

    #[test]
    fn sample() {
        let data = "16,1,2,0,4,2,7,1,2,14".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("37").as_deref());
        assert_eq!(p2.as_deref(), Some("168").as_deref());
    }
}

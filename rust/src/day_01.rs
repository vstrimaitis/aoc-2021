use crate::common::{get_nonempty_lines, Solver};

pub struct Solution;

impl Solver for Solution {
    fn solve(&self, input: &String) -> (Option<String>, Option<String>) {
        let arr: Vec<i32> = get_nonempty_lines(input)
            .map(|l| l.parse::<i32>().expect("Failed to parse line"))
            .collect();

        let ans1 = find_increase_count(&arr);
        let window_sums: Vec<i32> = arr.windows(3).map(|w| w.iter().sum()).collect();
        let ans2 = find_increase_count(&window_sums);
        (Some(ans1.to_string()), Some(ans2.to_string()))
    }
}

fn find_increase_count(arr: &Vec<i32>) -> i32 {
    arr.iter().zip(&arr[1..]).filter(|(x, y)| x < y).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let data = "199
200
208
210
200
207
240
269
260
263
"
        .to_string();
        let (p1, p2) = Solution.solve(&data);
        assert_eq!(p1.as_deref(), Some("7").as_deref());
        assert_eq!(p2.as_deref(), Some("5").as_deref());
    }
}

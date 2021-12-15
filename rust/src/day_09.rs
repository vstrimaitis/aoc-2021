use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    // parse input
    let board: Vec<Vec<i32>> = get_nonempty_lines(input)
        .map(|l| l
            .chars()
            .map(|c| c
                .to_digit(10)
                .expect("Failed to parse input")
                as i32
            )
            .collect()
        )
        .collect();
    let height = board.len();
    let width = board[0].len();

    // start actual solution
    let mut ans1 = 0;
    let mut visited = vec![vec![false; width]; height];

    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;

    for i in 0..height {
        for j in 0..width {
            let is_low_point =
                (i == 0 || board[i][j] < board[i-1][j]) &&
                (j == 0 || board[i][j] < board[i][j-1]) &&
                (i+1 >= height || board[i][j] < board[i+1][j]) &&
                (j+1 >= width || board[i][j] < board[i][j+1]);

            if is_low_point {
                ans1 += board[i][j] + 1;
            }
            if !visited[i][j] && board[i][j] != 9 {
                let size = dfs(&board, i, j, &mut visited);
                if size > max1 {
                    max3 = max2;
                    max2 = max1;
                    max1 = size;
                } else if size > max2 {
                    max3 = max2;
                    max2 = size;
                } else if size > max3 {
                    max3 = size;
                }
            }
        }
    }
    let ans2 = max1 * max2 * max3;

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn dfs(board: &Vec<Vec<i32>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
    if board[i][j] == 9 || visited[i][j] {
        return 0;
    }
    visited[i][j] = true;

    1 +
    if i > 0 { dfs(board, i-1, j, visited) } else { 0 } +
    if j > 0 { dfs(board, i, j-1, visited) } else { 0 } +
    if i+1 < board.len() { dfs(board, i+1, j, visited) } else { 0 } +
    if j+1 < board[0].len() { dfs(board, i, j+1, visited) } else { 0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/09.txt");
        b.iter(|| black_box(solve(&input.to_string())));
    }

    #[test]
    fn sample() {
        let data = "2199943210
3987894921
9856789892
8767896789
9899965678".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("15").as_deref());
        assert_eq!(p2.as_deref(), Some("1134").as_deref());
    }
}

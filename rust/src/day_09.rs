use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    // parse input
    let board: Vec<_> = get_nonempty_lines(input)
        .map(|l| l
            .chars()
            .map(|c| c
                .to_digit(10)
                .expect("Failed to parse input")
                as i32
            )
            .collect::<Vec<i32>>()
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
            let is_low_point = get_neighbors(&board, i, j)
                .iter()
                .all(|&(ii, jj)| board[i][j] < board[ii][jj]);
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
    visited[i][j] = true;
    let mut ans = 1;
    for (ii, jj) in get_neighbors(board, i, j) {
        if board[ii][jj] != 9 && !visited[ii][jj] {
            ans += dfs(board, ii, jj, visited);
        }
    }
    ans
}

fn get_neighbors(board: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let dis: Vec<i32> = vec![0, 1, 0, -1];
    let djs: Vec<i32> = vec![1, 0, -1, 0];
    dis.iter()
        .zip(djs.iter())
        .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
        .filter(|&(ii, jj)| ii >= 0 && ii < board.len() as i32 && jj >= 0 && jj < board[0].len() as i32)
        .map(|(ii, jj)| (ii as usize, jj as usize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
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

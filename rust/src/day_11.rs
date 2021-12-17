use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let mut board: Vec<Vec<u32>> = get_nonempty_lines(input)
        .map(|s| s.chars()
            .map(|c| c.to_digit(10).expect("Failed to parse input"))
            .collect()
        )
        .collect();
    let board_size = (board.len() * board[0].len()) as i32;
    let mut step = 0;
    let mut ans1 = 0;
    let mut ans2 = 0;

    while step <= 100 || ans2 == 0 {
        step += 1;
        let n_flashes = simulate(&mut board);
        if step <= 100 {
            ans1 += n_flashes;
        }
        if n_flashes == board_size {
            ans2 = step;
        }
    }

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn simulate(board: &mut Vec<Vec<u32>>) -> i32 {
    let n = board.len();
    let m = board[0].len();
    for i in 0..n {
        for j in 0..m {
            board[i][j] += 1
        }
    }

    let mut flashed = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if board[i][j] > 9 {
                flash(board, i, j, &mut flashed);
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if board[i][j] > 9 {
                board[i][j] = 0;
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            ans += flashed[i][j] as i32;
        }
    }
    ans
}

fn flash(board: &mut Vec<Vec<u32>>, i: usize, j: usize, flashed: &mut Vec<Vec<bool>>) {
    if flashed[i][j] {
        return;
    }
    flashed[i][j] = true;
    for di in [-1, 0, 1] {
        for dj in [-1, 0, 1] {
            if di == 0 && dj == 0 {
                continue;
            }
            let ii = i as i32 + di;
            let jj = j as i32 + dj;
            if 0 <= ii && ii < board.len() as i32 && 0 <= jj && jj < board[ii as usize].len() as i32 {
                board[ii as usize][jj as usize] += 1;
                if board[ii as usize][jj as usize] > 9 {
                    flash(board, ii as usize, jj as usize, flashed);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/11.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("1656").as_deref());
        assert_eq!(p2.as_deref(), Some("195").as_deref());
    }
}

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let mut board: Vec<Vec<char>> = input
        .split('\n')
        .filter(|&r| r.len() > 0)
        .map(|r| r.chars().collect())
        .collect();
    let mut ans1 = 1;
    while run_step(&mut board) {
        ans1 += 1;
    }
    (Some(ans1.to_string()), None)
}

fn run_step(board: &mut Vec<Vec<char>>) -> bool {
    let n = board.len();
    let m = board[0].len();
    let mut moved = false;
    for i in 0..n {
        let mut moves = Vec::with_capacity(m);
        for j in 0..m {
            let jj = (j + 1) % m;
            if board[i][j] == '>' && board[i][jj] == '.' {
                moves.push(j);
                moved = true;
            }
        }
        for &j in moves.iter() {
            let jj = (j + 1) % m;
            board[i][j] = '.';
            board[i][jj] = '>';
        }
    }

    for j in 0..m {
        let mut moves = Vec::with_capacity(n);
        for i in 0..n {
            let ii = (i + 1) % n;
            if board[i][j] == 'v' && board[ii][j] == '.' {
                moves.push(i);
                moved = true;
            }
        }
        for &i in moves.iter() {
            let ii = (i + 1) % n;
            board[i][j] = '.';
            board[ii][j] = 'v';
        }
    }

    moved
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/25.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
            .to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("58").as_deref());
        assert_eq!(p2, None);
    }
}

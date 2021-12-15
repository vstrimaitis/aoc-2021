use crate::common::*;
use std::collections::BinaryHeap;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let board: Vec<Vec<u8>> = get_nonempty_lines(input)
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Failed to parse input") as u8)
                .collect()
        })
        .collect();
    let ans1 = dijkstra(&board).expect("Failed to find path for part 1");
    let big_board = expand(&board);
    let ans2 = dijkstra(&big_board).expect("Failed to find path for part 2");
    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn dijkstra(board: &Vec<Vec<u8>>) -> Option<u16> {
    let n = board.len();
    let m = board[0].len();
    let mut dists: Vec<Vec<u16>> = vec![vec![u16::MAX; m]; n];
    let mut pq = BinaryHeap::new();

    dists[0][0] = 0;
    pq.push((0i16, (0, 0)));

    while let Some((dist, (r, c))) = pq.pop()
    {
        let dist = (-dist) as u16;
        if dist > dists[r][c] {
            continue;
        }
        if r == n - 1 && c == m - 1 {
            return Some(dist);
        }
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let rr = r as i16 + dr;
            let cc = c as i16 + dc;
            if rr < 0 || rr >= n as i16 || cc < 0 || cc >= m as i16 {
                continue;
            }
            let rr = rr as usize;
            let cc = cc as usize;
            let d = dist + board[rr][cc] as u16;
            if d < dists[rr][cc] {
                dists[rr][cc] = d;
                pq.push((-(d as i16), (rr, cc)));
            }
        }
    }
    None
}

fn expand(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let n = board.len();
    let m = board[0].len();
    let mut new_board = vec![vec![0; 5 * m]; 5 * n];
    for i in 0..5 * n {
        for j in 0..5 * m {
            let x = board[i % n][j % m] + (i / n) as u8 + (j / m) as u8;
            let x = (x - 1) % 9 + 1;
            new_board[i][j] = x;
        }
    }
    new_board
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let data = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("40").as_deref());
        assert_eq!(p2.as_deref(), Some("315").as_deref());
    }
}

use crate::common::*;
use std::collections::HashMap;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let (algo, board) = input.split_once("\n\n").expect("Failed to parse input");
    let algo: Vec<u8> = algo.chars().map(|c| (c == '#') as u8).collect();

    let outer_pixels = if algo.first().unwrap() == &1 && algo.last().unwrap() == &0 {
        [0, 1]
    } else {
        [0, 0]
    };

    let mut board = Board::from_string(board);
    for i in 0..2 {
        board.enhance(&algo, outer_pixels[i % 2]);
    }
    let ans1 = board.count_on();

    for i in 2..50 {
        board.enhance(&algo, outer_pixels[i % 2]);
    }
    let ans2 = board.count_on();
    (Some(ans1.to_string()), Some(ans2.to_string()))
}

static OFFSET: usize = 51;  // # of steps + 1

struct Board {
    values: [[u8; 210]; 210], // at least 2*OFFSET+100
    min_row: usize,
    max_row: usize,
    min_col: usize,
    max_col: usize,
}

impl Board {

    fn enhance(&mut self, algo: &Vec<u8>, outer: u8) {
        let mut new_values = Vec::with_capacity(300 * 300);
        self.min_row -= 1;
        self.max_row += 1;
        self.min_col -= 1;
        self.max_col += 1;
        for i in self.min_row..=self.max_row {
            for j in self.min_col..=self.max_col {
                let mut idx = 0u16;
                for di in -1i16..=1 {
                    for dj in -1i16..=1 {
                        let c = self.get((i as i16 + di) as usize, (j as i16 + dj) as usize, outer);
                        idx = 2 * idx + c as u16;
                    }
                }
                new_values.push((i, j, algo[idx as usize] as u8))
            }
        }
        for (i, j, x) in new_values {
            self.values[i][j] = x;
        }
    }

    fn count_on(&self) -> u16 {
        let mut ans = 0;
        for i in self.min_row..=self.max_row {
            for j in self.min_col..=self.max_col {
                ans += self.values[i][j] as u16;
            }
        }
        ans
    }

    fn get(&self, r: usize, c: usize, default: u8) -> u8 {
        if r <= self.min_row || r >= self.max_row || c <= self.min_col || c >= self.max_col {
            return default;
        }
        self.values[r][c]
    }

    fn from_string(s: &str) -> Board {
        let board: Vec<Vec<char>> = s
            .split('\n')
            .filter(|&r| r.len() > 0)
            .map(|r| r.chars().collect())
            .collect();
        let n = board.len();
        let m = board[0].len();
        let mut values = [[0u8; 210]; 210];

        for i in 0..n {
            for j in 0..m {
                values[i + OFFSET][j + OFFSET] = (board[i][j] == '#') as u8;
            }
        }

        Board {
            values: values,
            min_row: OFFSET + 0,
            max_row: OFFSET + n - 1,
            min_col: OFFSET + 0,
            max_col: OFFSET + m - 1,
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
        let input = include_str!("../../inputs/20.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("35").as_deref());
        assert_eq!(p2.as_deref(), Some("3351").as_deref());
    }
}

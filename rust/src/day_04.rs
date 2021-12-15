use std::time::Instant;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let start = Instant::now();
    let parts: Vec<_> = input.split("\n\n").map(|s| s.to_string()).collect();
    let nums: Vec<_> = parts[0].split(',').map(|x| x.parse::<i32>().expect("Failed to parse bingo number")).collect();
    let mut boards: Vec<Board> = parts[1..].iter().map(Board::parse).collect();

    println!("    > Parse duration: {:?}", start.elapsed());

    let mut ans1 = 0;
    let mut ans2 = 0;
    for num in nums.iter() {
        for b in boards.iter_mut() {
            b.mark(*num);
            if b.is_finished {
                ans2 = b.unmarked_sum * num;
                if ans1 == 0 {
                    ans1 = ans2;
                }
            }
        }
        boards = boards.into_iter().filter(|b| !b.is_finished).collect();
    }

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

struct Board {
    _n: usize,
    _m: usize,
    _values: Vec<Vec<i32>>,
    _coords: Vec<Option<(usize, usize)>>,
    _unmarked_col_counts: Vec<u32>,
    _unmarked_row_counts: Vec<u32>,
    _is_marked: Vec<Vec<bool>>,
    is_finished: bool,
    unmarked_sum: i32,
}

impl Board {
    fn parse(s: &String) -> Board {
        let values = s
            .split('\n')
            .filter(|l| l.len() > 0)
            .map(|l| l
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|x| x.parse::<i32>().expect("Failed to parse board value"))
                .collect::<Vec<i32>>()
            )
            .collect();
        Board::new(values)
    }

    fn new(values: Vec<Vec<i32>>) -> Board {
        let n = values.len();
        let m = values[0].len();
        let mut coords = vec![None; 100];
        let unmarked_col_counts = vec![m as u32; n];
        let unmarked_row_counts = vec![n as u32; m];
        let is_marked = vec![vec![false; m]; n];
        let mut unmarked_sum = 0;
        for i in 0..(n as usize) {
            for j in 0..(m as usize) {
                let x = values[i][j];
                coords[x as usize] = Some((i, j));
                unmarked_sum += x;
            }
        }
        Board { 
            _n: n,
            _m: m,
            _values: values.to_vec(),
            _coords: coords,
            _unmarked_col_counts: unmarked_col_counts,
            _unmarked_row_counts: unmarked_row_counts,
            _is_marked: is_marked,
            is_finished: false,
            unmarked_sum,
        }
    }

    fn mark(&mut self, num: i32) {
        if self._coords[num as usize].is_none() {
            return;
        }
        let (i, j) = self._coords[num as usize].unwrap();
        self._is_marked[i][j] = true;
        self._unmarked_col_counts[i] -= 1;
        self._unmarked_row_counts[j] -= 1;
        self.unmarked_sum -= num;
        if self._unmarked_row_counts[j] == 0 || self._unmarked_col_counts[i] == 0 {
            self.is_finished = true;
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
        let input = include_str!("../../inputs/04.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("4512").as_deref());
        assert_eq!(p2.as_deref(), Some("1924").as_deref());
    }
}

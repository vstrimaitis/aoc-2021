use crate::common::*;
use std::iter::repeat;
use std::ops;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let fish_nums: Vec<FishNumber> = get_nonempty_lines(input).map(|l| parse(l).0).collect();

    let f = fish_nums
        .iter()
        .fold(FishNumber::zero(), |a, b| a + b.clone());
    let ans1 = f.magnitude();

    let ans2 = fish_nums
        .iter()
        .flat_map(|a| repeat(a).zip(fish_nums.iter()))
        .filter(|&(a, b)| a != b)
        .map(|(a, b)| a.clone() + b.clone())
        .map(|f| f.magnitude())
        .max()
        .unwrap();

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn parse(s: &str) -> (FishNumber, &str) {
    if &s[0..1] == "[" {
        let (mut x, s) = parse(&s[1..]);
        assert_eq!(&s[0..1], ",");
        let (mut y, s) = parse(&s[1..]);
        assert_eq!(&s[0..1], "]");
        x.pair_positions.iter_mut().for_each(|p| p.push(0));
        y.pair_positions.iter_mut().for_each(|p| p.push(1));

        let mut values = x.values.to_vec();
        values.extend(y.values);
        let mut pair_positions = x.pair_positions.to_vec();
        pair_positions.extend(y.pair_positions);

        return (
            FishNumber {
                values,
                pair_positions,
            },
            &s[1..],
        );
    } else if "0123456789".contains(&s[0..1]) {
        let x = s[0..1].parse::<i16>().unwrap();
        return (
            FishNumber {
                values: vec![x],
                pair_positions: vec![vec![]],
            },
            &s[1..],
        );
    }
    unreachable!()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct FishNumber {
    values: Vec<i16>,
    pair_positions: Vec<Vec<u8>>,
}

impl FishNumber {
    pub fn zero() -> FishNumber {
        FishNumber {
            values: Vec::new(),
            pair_positions: Vec::new(),
        }
    }

    pub fn magnitude(&self) -> i16 {
        if self.len() == 1 {
            return self.values[0];
        }
        let (l, r) = self.to_pair();
        3 * l.magnitude() + 2 * r.magnitude()
    }

    fn to_pair(&self) -> (FishNumber, FishNumber) {
        let mut left = FishNumber::zero();
        let mut right = FishNumber::zero();
        for i in 0..self.len() {
            if self.pair_positions[i].last().unwrap() == &0 {
                assert_eq!(right.len(), 0);
                left.values.push(self.values[i]);
                left.pair_positions.push(self.pair_positions[i].to_vec());
                left.pair_positions.last_mut().unwrap().pop();
            } else {
                right.values.push(self.values[i]);
                right.pair_positions.push(self.pair_positions[i].to_vec());
                right.pair_positions.last_mut().unwrap().pop();
            }
        }
        (left, right)
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn concat(&self, other: &FishNumber) -> FishNumber {
        let mut f1 = self.clone();
        let mut f2 = other.clone();
        if f1.len() > 0 && f2.len() > 0 {
            f1.pair_positions.iter_mut().for_each(|p| p.push(0));
            f2.pair_positions.iter_mut().for_each(|p| p.push(1));
        }
        let mut f = f1.clone();
        f.values.extend(&f2.values);
        f.pair_positions.extend(f2.pair_positions);
        f
    }

    fn reduce(&self) -> Option<FishNumber> {
        self.explode().or_else(|| self.split())
    }

    fn explode(&self) -> Option<FishNumber> {
        for i in 0..self.len() - 1 {
            let pos = &self.pair_positions[i];
            let next_pos = &self.pair_positions[i + 1];
            if pos.len() > 4 && pos[0] == 0 && next_pos[0] == 1 {
                let (x, y) = (self.values[i], self.values[i + 1]);
                let mut left_val = self.values[..i].to_vec();
                let left_pos = &self.pair_positions[..i];
                let mut right_val = self.values[i + 2..].to_vec();
                let right_pos = &self.pair_positions[i + 2..];
                if left_val.len() > 0 {
                    *left_val.last_mut().unwrap() += x;
                }
                if right_val.len() > 0 {
                    *right_val.first_mut().unwrap() += y;
                }

                let mut values = left_val;
                values.push(0);
                values.extend(right_val);

                let mut pair_positions = left_pos.to_vec();
                pair_positions.push(pos[1..].to_vec());
                pair_positions.extend(right_pos.to_vec());

                return Some(FishNumber {
                    values,
                    pair_positions,
                });
            }
        }
        None
    }

    fn split(&self) -> Option<FishNumber> {
        for i in 0..self.len() {
            if self.values[i] >= 10 {
                let x = self.values[i] / 2;
                let y = self.values[i] / 2 + self.values[i] % 2;
                let left_val = self.values[..i].to_vec();
                let left_pos = self.pair_positions[..i].to_vec();
                let right_val = self.values[i + 1..].to_vec();
                let right_pos = self.pair_positions[i + 1..].to_vec();

                let mut values = left_val;
                values.push(x);
                values.push(y);
                values.extend(right_val);

                let mut pair_positions = left_pos;
                let mut pos_x = self.pair_positions[i].to_vec();
                pos_x.insert(0, 0);
                let mut pos_y = self.pair_positions[i].to_vec();
                pos_y.insert(0, 1);
                pair_positions.push(pos_x);
                pair_positions.push(pos_y);
                pair_positions.extend(right_pos);

                return Some(FishNumber {
                    values,
                    pair_positions,
                });
            }
        }
        None
    }
}

impl ops::Add<FishNumber> for FishNumber {
    type Output = FishNumber;

    fn add(self, rhs: FishNumber) -> FishNumber {
        let mut f = self.concat(&rhs);
        while let Some(ff) = f.reduce() {
            f = ff;
        }
        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/18.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            .to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("4140").as_deref());
        assert_eq!(p2.as_deref(), Some("3993").as_deref());
    }
}

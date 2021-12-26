use std::cmp::max;
// use memoize::memoize;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let (p1, p2) = parse_input(input);
    let ans1 = solve1(p1, p2, 100, 1000);
    let ans2 = solve2(p1, p2, 21);
    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn solve1(p1_pos: u8, p2_pos: u8, n_sides: u8, target_score: u16) -> u32 {
    let mut scores = [0u16, 0u16];
    let mut positions = [p1_pos, p2_pos];
    let mut turn = 0;
    let mut n_rolls = 0;
    let mut next_roll = 1u8;
    while scores.iter().all(|s| s < &target_score) {
        for _ in 0..3 {
            positions[turn] = mod1(positions[turn]+next_roll, 10);
            n_rolls += 1;
            next_roll = mod1(next_roll+1, n_sides);
        }
        scores[turn] += positions[turn] as u16;
        turn ^= 1;
    }
    *scores.iter().min().unwrap() as u32 * n_rolls as u32
}

fn solve2(p1_pos: u8, p2_pos: u8, target_score: u8) -> u64 {
    let mut dp = vec![vec![vec![vec![vec![(0u64, 0u64); 21]; 11]; 21]; 11]; 2];
    for s1 in (0..target_score as usize).rev() {
        for s2 in (0..target_score as usize).rev() {
            for p1 in 1..=10usize {
                for p2 in 1..=10usize {
                    for r1 in [1, 2, 3] {
                        for r2 in [1, 2, 3] {
                            for r3 in [1, 2, 3] {
                                let r = mod1(r1+r2+r3, 10);
                                let new_p1 = mod1(p1 as u8+r, 10) as usize;
                                let new_s1 = s1 + new_p1;
                                let new_p2 = mod1(p2 as u8+r, 10) as usize;
                                let new_s2 = s2 + new_p2;
                                let next0 = if (new_s1 as u8) < target_score {
                                    dp[1][new_p1][new_s1][p2][s2]
                                } else {
                                    (1, 0)
                                };
                                let next1 = if (new_s2 as u8) < target_score {
                                    dp[0][p1][s1][new_p2][new_s2]
                                } else {
                                    (0, 1)
                                };
                                dp[0][p1][s1][p2][s2] = add(&dp[0][p1][s1][p2][s2], &next0);
                                dp[1][p1][s1][p2][s2] = add(&dp[1][p1][s1][p2][s2], &next1);
                            }
                        }
                    }
                }
            }
        }
    }
    let (w1, w2) = dp[0][p1_pos as usize][0][p2_pos as usize][0];
    max(w1, w2)
    // let (p1_wins, p2_wins) = dp(0, 0, p1_pos, 0, p2_pos, target_score);
    // return max(p1_wins, p2_wins)
}

fn add(t1: &(u64, u64), t2: &(u64, u64)) -> (u64, u64){
    let (x, y) = t1;
    let (a, b) = t2;
    (x+a, y+b)
}

// #[memoize]
// fn dp(turn: u8, p1_score: u8, p1_pos: u8, p2_score: u8, p2_pos: u8, target_score: u8) -> (u64, u64) {
//     if p1_score >= target_score {
//         return (1, 0);
//     }
//     if p2_score >= target_score {
//         return (0, 1);
//     }
//     let mut p1_wins = 0;
//     let mut p2_wins = 0;
//     for r1 in [1, 2, 3] {
//         for r2 in [1, 2, 3] {
//             for r3 in [1, 2, 3] {
//                 let r = mod1(r1+r2+r3, 10);
//                 if turn == 0 {
//                     let new_pos = mod1(p1_pos+r, 10);
//                     let new_score = p1_score + new_pos;
//                     let (s1, s2) = dp(1, new_score, new_pos, p2_score, p2_pos, target_score);
//                     p1_wins += s1;
//                     p2_wins += s2;
//                 } else {
//                     let new_pos = mod1(p2_pos+r, 10);
//                     let new_score = p2_score + new_pos;
//                     let (s1, s2) = dp(0, p1_score, p1_pos, new_score, new_pos, target_score);
//                     p1_wins += s1;
//                     p2_wins += s2;
//                 }
//             }
//         }
//     }
//     (p1_wins, p2_wins)
// }

fn mod1(x: u8, m: u8) -> u8 {
    (x-1)%m+1
}

fn parse_input(s: &String) -> (u8, u8) {
    let positions: Vec<u8> = s.split('\n')
        .filter(|&r| r.len() > 0)
        .map(|r| r.split_once(": ").expect("Failed to parse line"))
        .map(|(_, p)| p.parse::<u8>().expect("Failed to parse starting position"))
        .collect();
    assert_eq!(positions.len(), 2);
    (positions[0], positions[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/21.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "Player 1 starting position: 4
Player 2 starting position: 8".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("739785").as_deref());
        assert_eq!(p2.as_deref(), Some("444356092776315").as_deref());
    }
}

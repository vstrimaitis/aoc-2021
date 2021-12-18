use crate::common::*;
use std::iter::{once, repeat};

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let fish_nums: Vec<Vec<Token>> = get_nonempty_lines(input).map(parse).collect();
    let ans1 = magnitude(
        &mut fish_nums
            .iter()
            .fold(Vec::new(), |a, b| add(a, b.clone()))
            .into_iter(),
    );
    let ans2 = fish_nums
        .iter()
        .enumerate()
        .flat_map(|(i, x)| repeat((i, x)).zip(fish_nums.iter().enumerate()))
        .filter_map(|((i, x), (j, y))| {
            if i == j {
                None
            } else {
                Some((x.clone(), y.clone()))
            }
        })
        .map(|(a, b)| add(a, b))
        .map(|x| magnitude(&mut x.clone().into_iter()))
        .max()
        .unwrap();

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    Open,
    Close,
    Num(u16),
}

fn magnitude<I>(tokens: &mut I) -> u16
where
    I: Iterator<Item = Token>,
{
    match tokens.next().unwrap() {
        Token::Open => {
            let left = magnitude(tokens);
            let right = magnitude(tokens);
            let next = tokens.next().unwrap();
            assert_eq!(next, Token::Close);
            3 * left + 2 * right
        }
        Token::Num(x) => x,
        _ => unreachable!(),
    }
}

fn parse(s: &str) -> Vec<Token> {
    s.chars()
        .filter_map(|c| match c {
            '[' => Some(Token::Open),
            ']' => Some(Token::Close),
            ',' => None,
            _ => Some(Token::Num(
                c.to_digit(10).expect("Failed to parse input") as u16
            )),
        })
        .collect()
}

fn add(a: Vec<Token>, b: Vec<Token>) -> Vec<Token> {
    if a.len() == 0 {
        return b;
    }
    if b.len() == 0 {
        return a;
    }
    let mut res = concat(&a, &b);
    loop {
        let mut nesting = 0;
        let mut changed = false;
        for i in 0..res.len() {
            if i+3 >= res.len() {
                break;
            }
            if nesting > 3 && try_explode(&mut res, i) {
                changed = true;
                break;
            }
            match res[i] {
                Token::Open => nesting += 1,
                Token::Close => nesting -= 1,
                _ => (),
            };
        }
        if !changed {
            for i in 0..res.len() {
                if try_split(&mut res, i) {
                    changed = true;
                    break;
                }
            }
        }
        if !changed {
            break;
        }
    }
    res
}

fn try_split(tokens: &mut Vec<Token>, i: usize) -> bool {
    match tokens[i] {
        Token::Num(x) if x >= 10 => {
            tokens.splice(i..i+1, [
                Token::Open,
                Token::Num(x / 2),
                Token::Num(x / 2 + x % 2),
                Token::Close,
            ]);
            true
        }
        _ => false,
    }
}

fn try_explode(tokens: &mut Vec<Token>, i: usize) -> bool {
    match tokens[i..i+4] {
        [Token::Open, Token::Num(x), Token::Num(y), Token::Close] => {
            for j in (0..i).rev() {
                match tokens[j] {
                    Token::Num(z) => {
                        tokens[j] = Token::Num(z + x);
                        break;
                    }
                    _ => (),
                };
            }
            for j in i+4..tokens.len() {
                match tokens[j] {
                    Token::Num(z) => {
                        tokens[j] = Token::Num(z + y);
                        break;
                    }
                    _ => (),
                };
            }
            tokens.splice(i..i+4, once(Token::Num(0)));
            true
        }
        _ => false
    }
}

fn concat(a: &Vec<Token>, b: &Vec<Token>) -> Vec<Token> {
    once(&Token::Open)
        .chain(&a[..])
        .chain(&b[..])
        .chain(once(&Token::Close))
        .map(|x| x.clone())
        .collect()
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

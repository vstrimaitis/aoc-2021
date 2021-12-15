use crate::common::*;

#[derive(PartialEq)]
enum ResultType {
    SyntaxError,
    Incomplete,
}

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let analysis_results: Vec<_> = get_nonempty_lines(input)
        .map(analyze_line)
        .collect();
    let ans1: i64 = analysis_results
        .iter()
        .filter(|(r, _)| r == &ResultType::SyntaxError)
        .map(|(_, score)| score)
        .sum();

    let mut completion_scores: Vec<_> = analysis_results
        .iter()
        .filter(|(r, _)| r == &ResultType::Incomplete)
        .map(|(_, score)| score)
        .collect();
    completion_scores.sort();
    let ans2 = completion_scores[completion_scores.len() / 2];

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn analyze_line(line: &str) -> (ResultType, i64) {
    let mut opens = vec![];
    for c in line.chars() {
        if is_opening(c) {
            opens.push(c);
        } else {
            let matching = opens.pop().expect("No open brackets available");
            if flip(&c) != matching {
                return (ResultType::SyntaxError, calc_syntax_error_score(c));
            }
        }
    }
    opens.reverse();
    (ResultType::Incomplete, calc_completion_score(opens.iter().map(flip).collect::<String>()))
}

fn calc_completion_score(suffix: String) -> i64 {
    let mut ans = 0;
    for c in suffix.chars() {
        ans = 5*ans + match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unrecognized character: {}", c),
        }
    }
    ans
}

fn calc_syntax_error_score(last: char) -> i64 {
    match last {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unrecognized character: {}", last),
    }
}

fn is_opening(c: char) -> bool {
    "([{<".contains(c)
}

fn flip(c: &char) -> char {
    let pairs = vec![
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ];
    for (open, closed) in pairs.iter() {
        if c == open {
            return *closed;
        }
        if c == closed {
            return *open;
        }
    }
    panic!("Failed to flip bracket");
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/10.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("26397").as_deref());
        assert_eq!(p2.as_deref(), Some("288957").as_deref());
    }
}

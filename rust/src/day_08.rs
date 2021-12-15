use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let (ans1, ans2) = get_nonempty_lines(input)
        .map(parse_line)
        .map(solve_line)
        .fold((0, 0), |(acc1, acc2), (x1, x2)| (acc1+x1, acc2+x2));

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn parse_line(line: &str) -> (Vec<u8>, Vec<u8>) {
    let parts: Vec<_> = line.split(" | ")
        .map(parse_half)
        .collect();
    assert!(parts.len() == 2);
    (parts[0].to_vec(), parts[1].to_vec())
}

fn solve_line((observed_signals, output_signals): (Vec<u8>, Vec<u8>)) -> (i32, i32) {
    // part 1
    let ans1 = output_signals.iter()
        .map(|s| s.count_ones())
        .filter(|&bitcount| bitcount == 2 || bitcount == 3 || bitcount == 4 || bitcount == 7)
        .count() as i32;

    // part 2
    let mut by_length: Vec<Vec<u8>> = vec![vec![]; 8];
    for s in observed_signals {
        by_length[s.count_ones() as usize].push(s)
    }
    let mut digit_to_pattern = vec![0u8; 10];
    digit_to_pattern[1] = by_length[2][0];
    digit_to_pattern[7] = by_length[3][0];
    digit_to_pattern[4] = by_length[4][0];
    digit_to_pattern[8] = by_length[7][0];
    digit_to_pattern[3] = pick(&mut by_length[5], |s| is_subset(&digit_to_pattern[1], s));
    digit_to_pattern[9] = pick(&mut by_length[6], |s| is_subset(&digit_to_pattern[4], s));
    digit_to_pattern[0] = pick(&mut by_length[6], |s| is_subset(&digit_to_pattern[1], s));
    digit_to_pattern[6] = pick(&mut by_length[6], |_| true);
    digit_to_pattern[5] = pick(&mut by_length[5], |s| is_subset(s, &digit_to_pattern[9]));
    digit_to_pattern[2] = pick(&mut by_length[5], |_| true);

    let ans2 = output_signals.iter()
        .map(|s| digit_to_pattern.iter()
            .position(|t| s == t)
            .expect("Failed to decode digit")
            as i32
        )
        .fold(0, |acc, d| 10*acc + d);

    (ans1, ans2)
}

fn pick<F>(patterns: &mut Vec<u8>, predicate_fn: F) -> u8 where
    F: Fn(&u8) -> bool {
    let i = patterns.iter()
        .position(|p| predicate_fn(p))
        .expect("Failed to deduce digit");
    let ans = patterns[i].to_owned();
    patterns.swap_remove(i);
    ans
}

fn is_subset(s: &u8, t: &u8) -> bool {
    &(s & t) == s
}

fn parse_half(half: &str) -> Vec<u8> {
    // Represent each string as an 8-bit integer (with only 7 bits that are actually used)
    half.split(' ')
        .map(to_bitset)
        .collect()
}

fn to_bitset(s: &str) -> u8 {
    let mut ans = 0;
    for c in s.chars() {
        let i = c as u8 - 'a' as u8;
        ans += 1 << i;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/08.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample() {
        let data = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("26").as_deref());
        assert_eq!(p2.as_deref(), Some("61229").as_deref());
    }
}

use crate::common::*;
use std::collections::HashMap;
use std::str;

static PADDING: char = '$';
static N_AVAILABLE_CHARS: usize = 11;
static MAX_PAIRS: usize = N_AVAILABLE_CHARS * N_AVAILABLE_CHARS;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let (polymer_template, rules) = input.split_once("\n\n").expect("Failed to parse input");

    let polymer_template = [String::from(PADDING), polymer_template.to_string(), String::from(PADDING)].join("");
    let rules: HashMap<(char, char), char> = get_nonempty_lines(&rules.to_string())
        .map(|l| l.split_once(" -> ").expect("Failed to parse rule"))
        .map(parse_rule)
        .collect();

    let mut freq_table = build_freq_table(&polymer_template);
    
    let mut ans1 = 0;
    for step in 0..40 {
        if step == 10 {
            ans1 = calc_answer(&freq_table);
        }
        freq_table = apply_rules(&freq_table, &rules);
    }
    let ans2 = calc_answer(&freq_table);

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn parse_rule((from, to): (&str, &str)) -> ((char, char), char) {
    let mut chars = from.chars();
    let from_1 = chars.next().expect("Failed to parse rule key");
    let from_2 = chars.next().expect("Failed to parse rule key");
    let to = to.chars().next().expect("Failed to parse rule");
    ((from_1, from_2), to)
}

fn calc_answer(freq_table: &Vec<u64>) -> u64 {
    let mut char_counts: Vec<u64> = vec![0; 30];
    for i in 0..MAX_PAIRS {
        let f = freq_table[i as usize];
        if f == 0 {
            continue;
        }
        let (a, b) = from_int(i as u16);
        if a != PADDING {
            let a_id = a as u8 - 'A' as u8;
            char_counts[a_id as usize] += f;
        }
        if b != PADDING {
            let b_id = b as u8 - 'A' as u8;
            char_counts[b_id as usize] += f;
        }
    }
    let counts: Vec<_> = char_counts.into_iter().filter(|&x| x > 0).collect();
    let mx = counts.iter().max().unwrap();
    let mn = counts.iter().min().unwrap();

    (mx - mn) / 2
}

fn to_int((a, b): (char, char)) -> u16 {
    let convert = |c| {
        match c {
            '$' => 0,
            'B' => 1,
            'C' => 2,
            'F' => 3,
            'H' => 4,
            'K' => 5,
            'N' => 6,
            'O' => 7,
            'P' => 8,
            'S' => 9,
            'V' => 10,
            _ => unreachable!(),
        }
    };
    convert(a) * N_AVAILABLE_CHARS as u16 + convert(b)
}

fn from_int(i: u16) -> (char, char) {
    let convert = |x| {
        match x {
            0 => PADDING,
            1 => 'B',
            2 => 'C',
            3 => 'F',
            4 => 'H',
            5 => 'K',
            6 => 'N',
            7 => 'O',
            8 => 'P',
            9 => 'S',
            10 => 'V',
            _ => unreachable!(),
        }
    };
    (convert((i / N_AVAILABLE_CHARS as u16) as u8), convert((i % N_AVAILABLE_CHARS as u16) as u8))
}

fn apply_rules(freq_table: &Vec<u64>, rules: &HashMap<(char, char), char>) -> Vec<u64> {
    let mut new_freqs: Vec<u64> = vec![0; MAX_PAIRS];

    for i in 0..MAX_PAIRS {
        let (a, b) = from_int(i as u16);
        let f = freq_table[i as usize];
        if rules.contains_key(&(a, b)) {
            let mid = rules[&(a, b)];
            let i1 = to_int((a, mid));
            let i2 = to_int((mid, b));
            new_freqs[i1 as usize] += f;
            new_freqs[i2 as usize] += f;
        } else {
            new_freqs[i as usize] += f;
        }
    }

    new_freqs
}

fn build_freq_table(s: &String) -> Vec<u64> {
    let pairs: Vec<_> = s.as_bytes()
        .windows(2)
        .map(|x| (x[0] as char, x[1] as char))
        .collect();
    let mut freqs: Vec<u64> = vec![0; MAX_PAIRS];
    for &p in pairs.iter() {
        let id = to_int(p);
        freqs[id as usize] += 1;
    }
    freqs
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/14.txt");
        b.iter(|| black_box(solve(&input.to_string())));
    }

    #[test]
    fn sample() {
        let data = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("1588").as_deref());
        assert_eq!(p2.as_deref(), Some("2188189693529").as_deref());
    }
}

use crate::common::*;
use std::collections::HashMap;
use std::str;

static PADDING: char = '$';

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

fn calc_answer(freq_table: &HashMap<String, u64>) -> u64 {
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for (pair, f) in freq_table.iter() {
        let mut chars = pair.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        *char_counts.entry(a).or_default() += f;
        *char_counts.entry(b).or_default() += f;
    }
    char_counts.remove_entry(&PADDING);
    let mn = char_counts.values().min().unwrap() / 2;
    let mx = char_counts.values().max().unwrap() / 2;

    mx - mn
}

fn apply_rules(freq_table: &HashMap<String, u64>, rules: &HashMap<(char, char), char>) -> HashMap<String, u64> {
    let mut new_freqs: HashMap<String, u64> = HashMap::new();

    for (pair, &f) in freq_table.iter() {
        let mut chars = pair.chars();
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        if rules.contains_key(&(a, b)) {
            let mid = rules[&(a, b)];
            let p1 = String::from_iter([a, mid]);
            let p2 = String::from_iter([mid, b]);
            *new_freqs.entry(p1).or_default() += f;
            *new_freqs.entry(p2).or_default() += f;
        } else {
            *new_freqs.entry(pair.to_string()).or_default() += f;
        }
    }

    new_freqs
}

fn build_freq_table(s: &String) -> HashMap<String, u64> {
    let pairs: Vec<_> = s.as_bytes()
        .windows(2)
        .map(|x| str::from_utf8(x).expect("Failed to split string into pairs"))
        .collect();
    let mut freqs: HashMap<String, u64> = HashMap::new();
    for p in pairs.iter() {
        *freqs.entry(p.to_string()).or_default() += 1;
    }
    freqs
}

#[cfg(test)]
mod tests {
    use super::*;
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

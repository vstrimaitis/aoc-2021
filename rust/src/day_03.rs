use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let bitstrings: Vec<_> = get_nonempty_lines(input)
        .map(|s| s.to_string())
        .collect();
    let numbers: Vec<_> = bitstrings
        .iter()
        .map(|s| bitstring_to_int(&s))
        .collect();
    let n_bits = bitstrings[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;
    let mut oxygen = numbers.to_vec();
    let mut co2 = numbers.to_vec();
    for i in (0..n_bits).rev() {
        let mcb = get_most_common_bit(&numbers, i);
        gamma = 2 * gamma + mcb;
        epsilon = 2 * epsilon + mcb;
        oxygen = reduce_candidates(oxygen, i, |mcb| if mcb == 0 { 0 } else { 1 });
        co2 = reduce_candidates(co2, i, |mcb| if mcb == 0 { 1 } else { 0 });
    }
    let ans1 = gamma * epsilon;
    let ans2 = oxygen[0] * co2[0];

    return (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn get_bit(x: u32, index: usize) -> u32 {
    if (x & (1 << index)) > 0 {
        1
    } else {
        0
    }
}

fn bitstring_to_int(bs: &String) -> u32 {
    u32::from_str_radix(&bs, 2).unwrap()
}

fn get_most_common_bit(numbers: &Vec<u32>, index: usize) -> u32 {
    let zero_count = numbers
        .iter()
        .map(|&x| get_bit(x, index))
        .filter(|&b| b == 0)
        .count();
    let one_count = numbers.len() - zero_count;
    if zero_count > one_count {
        0
    } else {
        1
    }
}

fn reduce_candidates(numbers: Vec<u32>, index: usize, bit_criteria: fn(u32) -> u32) -> Vec<u32> {
    if numbers.len() == 1 {
        return numbers;
    }
    let mcb = get_most_common_bit(&numbers, index);
    let target = bit_criteria(mcb);
    numbers
        .into_iter()
        .filter(|&x| get_bit(x, index) == target)
        .collect()
}

#[allow(dead_code)]
pub fn solve_slow(input: &String) -> (Option<String>, Option<String>) {
    let bitstrings: Vec<_> = get_nonempty_lines(input).map(|s| s.to_string()).collect();
    let n_bits = bitstrings[0].len();

    let most_common_bits: Vec<_> = (0..n_bits)
        .map(|i| get_most_common_bit_str(&bitstrings, i))
        .collect();

    let gamma = most_common_bits
        .iter()
        .map(|&b| char::from_digit(b, 10).unwrap())
        .collect::<String>();

    let epsilon = most_common_bits
        .iter()
        .map(|&b| b^1)
        .map(|b| char::from_digit(b, 10).unwrap())
        .collect::<String>();

    let ans1 = isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap();

    let mut oxygen = bitstrings.to_vec();
    let mut co2 = bitstrings.to_vec();
    for i in 0..n_bits {
        oxygen = reduce_candidates_str(oxygen, i, |mcb| if mcb == 0 { '0' } else { '1' });
        co2 = reduce_candidates_str(co2, i, |mcb| if mcb == 0 { '1' } else { '0' });
    }
    let ans2 = isize::from_str_radix(&oxygen[0], 2).unwrap() * isize::from_str_radix(&co2[0], 2).unwrap();

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn reduce_candidates_str(bitstrings: Vec<String>, index: usize, bit_criteria: fn(u32) -> char) -> Vec<String> {
    if bitstrings.len() == 1 {
        return bitstrings;
    }
    let mcb = get_most_common_bit_str(&bitstrings, index);
    let target = bit_criteria(mcb);
    bitstrings
        .into_iter()
        .filter(|b| b.chars().nth(index).unwrap() == target)
        .collect()
}

fn get_most_common_bit_str(bitstrings: &Vec<String>, index: usize) -> u32 {
    let zero_count = bitstrings
        .iter()
        .map(|s| s.chars().nth(index).unwrap())
        .filter(|&c| c == '0')
        .count();
    let one_count = bitstrings.len() - zero_count;
    if zero_count > one_count {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let data = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("198").as_deref());
        assert_eq!(p2.as_deref(), Some("230").as_deref());
    }
}

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let input_bin = hex_to_bin(&input);
    let (packet, rest) = parse(&input_bin);
    assert!(rest.chars().all(|c| c == '0'));

    let ans1 = calc_version_sum(&packet);
    let ans2 = eval(&packet);
    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn hex_to_bin(s: &str) -> String {
    s.trim()
        .chars()
        .map(char_to_bin)
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Debug, Eq, PartialEq)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, Eq, PartialEq)]
struct PacketHeader {
    type_id: u8,
    version: u8,
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal(PacketHeader, i64),
    Operator(PacketHeader, OperatorType, Vec<Packet>),
}

fn char_to_bin(c: char) -> String {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Invalid character found: '{}'", c),
    }
    .to_string()
}

fn parse(s: &str) -> (Packet, &str) {
    let (header, s) = take_header(s);
    match header.type_id {
        4 => take_literal(s, header),
        _ => take_operator(s, header),
    }
}

fn take_operator(s: &str, header: PacketHeader) -> (Packet, &str) {
    let (l, s) = take_length_type_id(s);
    let op_type = determine_operator_type(header.type_id);
    let (subpackets, s) = match l {
        0 => take_subpackets_by_length(s),
        1 => take_subpackets_by_count(s),
        _ => unreachable!(),
    };
    (Packet::Operator(header, op_type, subpackets), s)
}

fn determine_operator_type(type_id: u8) -> OperatorType {
    match type_id {
        0 => OperatorType::Sum,
        1 => OperatorType::Product,
        2 => OperatorType::Minimum,
        3 => OperatorType::Maximum,
        5 => OperatorType::GreaterThan,
        6 => OperatorType::LessThan,
        7 => OperatorType::EqualTo,
        _ => unreachable!(),
    }
}

fn take_subpackets_by_length(s: &str) -> (Vec<Packet>, &str) {
    let (len, s) = take_int(s, 15);
    let len = len as usize;
    let (mut s_subpackets, s) = take_prefix(s, len); 

    let mut subpackets = Vec::new();
    while s_subpackets.len() > 0 {
        let (p, ss) = parse(s_subpackets);
        subpackets.push(p);
        s_subpackets = ss;
    }

    (subpackets, s)
}

fn take_subpackets_by_count(s: &str) -> (Vec<Packet>, &str) {
    let (cnt, s) = take_int(s, 11);
    let cnt = cnt as usize;
    let mut rest = s;
    let mut subpackets = Vec::with_capacity(cnt);
    for _ in 0..cnt {
        let (p, ss) = parse(rest);
        subpackets.push(p);
        rest = ss;
    }
    (subpackets, rest)
}

fn take_length_type_id(s: &str) -> (u8, &str) {
    let (l, s) = take_int(s, 1);
    (l as u8, s)
}

fn take_literal(s: &str, header: PacketHeader) -> (Packet, &str) {
    let mut final_value = 0;
    let mut s = s;
    loop {
        let ((value, is_last), rest) = take_literal_group(s);
        s = rest;
        final_value = 16 * final_value + value;
        if !is_last {
            break;
        }
    }
    (Packet::Literal(header, final_value), s)
}

fn take_literal_group(s: &str) -> ((i64, bool), &str) {
    let (first, s) = take_int(s, 1);
    let flag = first == 1;
    let (val, s) = take_int(s, 4);
    ((val, flag), s)
}

fn take_header(s: &str) -> (PacketHeader, &str) {
    let (version, s) = take_int(s, 3);
    let (type_id, s) = take_int(s, 3);
    (PacketHeader { version: version as u8, type_id: type_id as u8 }, s)
}

fn take_int(s: &str, num_bits: usize) -> (i64, &str) {
    let x = i64::from_str_radix(&s[..num_bits], 2).unwrap();
    (x, &s[num_bits..])
}

fn take_prefix(s: &str, num_bits: usize) -> (&str, &str) {
    (&s[..num_bits], &s[num_bits..])
}

fn calc_version_sum(p: &Packet) -> u16 {
    match p {
        Packet::Literal(PacketHeader { version, .. }, _) => *version as u16,
        Packet::Operator(PacketHeader { version, .. }, _, args) => {
            *version as u16 + args.iter().map(calc_version_sum).sum::<u16>()
        }
    }
}

fn eval(p: &Packet) -> i64 {
    match p {
        Packet::Literal(_, value) => *value,
        Packet::Operator(_, op_type, args) => {
            let subvalues = args.iter().map(eval);
            match op_type {
                OperatorType::Sum => subvalues.sum(),
                OperatorType::Product => subvalues.product(),
                OperatorType::Minimum => subvalues.min().unwrap(),
                OperatorType::Maximum => subvalues.max().unwrap(),
                OperatorType::GreaterThan => subvalues
                    .collect::<Vec<i64>>()
                    .windows(2)
                    .all(|w| w[0] > w[1]) as i64,
                OperatorType::LessThan => subvalues
                    .collect::<Vec<i64>>()
                    .windows(2)
                    .all(|w| w[0] < w[1]) as i64,
                OperatorType::EqualTo => subvalues
                    .collect::<Vec<i64>>()
                    .windows(2)
                    .all(|w| w[0] == w[1]) as i64,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OperatorType::*;
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/16.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample_part1_1() {
        let data = "8A004A801A8002F478".to_string();
        let (p1, _) = solve(&data);
        assert_eq!(p1.as_deref(), Some("16").as_deref());
    }

    #[test]
    fn sample_part1_2() {
        let data = "620080001611562C8802118E34".to_string();
        let (p1, _) = solve(&data);
        assert_eq!(p1.as_deref(), Some("12").as_deref());
    }

    #[test]
    fn sample_part1_3() {
        let data = "C0015000016115A2E0802F182340".to_string();
        let (p1, _) = solve(&data);
        assert_eq!(p1.as_deref(), Some("23").as_deref());
    }
    #[test]
    fn sample_part1_4() {
        let data = "A0016C880162017C3686B18A3D4780".to_string();
        let (p1, _) = solve(&data);
        assert_eq!(p1.as_deref(), Some("31").as_deref());
    }

    #[test]
    fn sample_part2_1() {
        let data = "C200B40A82".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("3").as_deref());
    }

    #[test]
    fn sample_part2_2() {
        let data = "04005AC33890".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("54").as_deref());
    }

    #[test]
    fn sample_part2_3() {
        let data = "880086C3E88112".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("7").as_deref());
    }

    #[test]
    fn sample_part2_4() {
        let data = "CE00C43D881120".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("9").as_deref());
    }

    #[test]
    fn sample_part2_5() {
        let data = "D8005AC2A8F0".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("1").as_deref());
    }

    #[test]
    fn sample_part2_6() {
        let data = "F600BC2D8F".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("0").as_deref());
    }

    #[test]
    fn sample_part2_7() {
        let data = "9C005AC2F8F0".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("0").as_deref());
    }

    #[test]
    fn sample_part2_8() {
        let data = "9C0141080250320F1802104A08".to_string();
        let (_, p2) = solve(&data);
        assert_eq!(p2.as_deref(), Some("1").as_deref());
    }

    #[test]
    fn test_parse() {
        let test_cases: Vec<(&str, usize, Packet)> = vec![
            ("D2FE28", 3, lit(6, 2021)),
            (
                "38006F45291200",
                7,
                op(1, LessThan, vec![lit(6, 10), lit(2, 20)]),
            ),
            (
                "EE00D40C823060",
                5,
                op(7, Maximum, vec![lit(2, 1), lit(4, 2), lit(1, 3)]),
            ),
            (
                "8A004A801A8002F478",
                3,
                op(
                    4,
                    Minimum,
                    vec![op(1, Minimum, vec![op(5, Minimum, vec![lit(6, 15)])])],
                ),
            ),
            (
                "620080001611562C8802118E34",
                2,
                op(
                    3,
                    Sum,
                    vec![
                        op(0, Sum, vec![lit(0, 10), lit(5, 11)]),
                        op(1, Sum, vec![lit(0, 12), lit(3, 13)]),
                    ],
                ),
            ),
            (
                "C0015000016115A2E0802F182340",
                6,
                op(
                    6,
                    Sum,
                    vec![
                        op(0, Sum, vec![lit(0, 10), lit(6, 11)]),
                        op(4, Sum, vec![lit(7, 12), lit(0, 13)]),
                    ],
                ),
            ),
            (
                "A0016C880162017C3686B18A3D4780",
                7,
                op(
                    5,
                    Sum,
                    vec![op(
                        1,
                        Sum,
                        vec![op(
                            3,
                            Sum,
                            vec![lit(7, 6), lit(6, 6), lit(5, 12), lit(2, 15), lit(2, 15)],
                        )],
                    )],
                ),
            ),
        ];
        for (input, num_zeros_left, expected_packet) in test_cases {
            let input_bin = hex_to_bin(input);
            let (packet, rest) = parse(&input_bin);
            assert!(rest.chars().all(|c| c == '0'));
            assert_eq!(rest.len(), num_zeros_left);
            assert_eq!(packet, expected_packet);
        }
    }

    #[test]
    fn test_evaluate() {
        let test_cases: Vec<(&str, i64)> = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (input, expected_val) in test_cases {
            let input_bin = hex_to_bin(input);
            let (packet, _) = parse(&input_bin);
            let val = eval(&packet);
            assert_eq!(val, expected_val);
        }
    }
    fn lit(version: u8, value: i64) -> Packet {
        Packet::Literal(
            PacketHeader {
                version,
                type_id: 4,
            },
            value,
        )
    }

    fn op(version: u8, op_type: OperatorType, args: Vec<Packet>) -> Packet {
        let type_id = match op_type {
            Sum => 0,
            Product => 1,
            Minimum => 2,
            Maximum => 3,
            GreaterThan => 5,
            LessThan => 6,
            EqualTo => 7,
        };
        Packet::Operator(PacketHeader { version, type_id }, op_type, args)
    }
}

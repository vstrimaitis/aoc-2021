use Register::*;
use Operand::*;
use Instruction::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let program = parse_program(input);
    let mut stack = Vec::new();
    let mut pairs = [(0, 0); 14];
    for i in 0..14 {
        let important_ops = (
            program[18*i+4],
            program[18*i+5],
            program[18*i+15]
        );
        let (div_z, c1, c2) = match important_ops {
            (Div(Z, Value(div_z)), Add(X, Value(c1)), Add(Y, Value(c2))) => (div_z, c1, c2),
            _ => unreachable!()
        };
        if div_z == 1 {
            stack.push((i, c1, c2));
        } else {
            let (prev_i, _, prev_c2) = stack.pop().unwrap();
            let delta = prev_c2 + c1;
            pairs[prev_i] = (delta, i);
            pairs[i] = (-delta, prev_i);
        }
    }

    let mut digits_max = [0; 14];
    let mut digits_min = [0; 14];

    for i in 0..14 {
        if digits_max[i] > 0 {
            continue;
        }
        let (delta, j) = pairs[i];
        if delta < 0 {
            digits_max[i] = 9;
            digits_max[j] = digits_max[i] + delta;
            digits_min[j] = 1;
            digits_min[i] = digits_min[j] - delta; 
        } else {
            digits_max[j] = 9;
            digits_max[i] = digits_max[j] - delta;
            digits_min[i] = 1;
            digits_min[j] = digits_min[i] + delta;
        }
    }

    let mut ans1 = 0u64;
    let mut ans2 = 0u64;
    for i in 0..14 {
        ans1 = 10*ans1 + digits_max[i] as u64;
        ans2 = 10*ans2 + digits_min[i] as u64;
    }

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

#[derive(Debug, Copy, Clone)]
enum Register {
    X,
    Y,
    Z,
    W,
}

impl Register {
    fn from_string(s: &str) -> Register {
        match s {
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            "w" => Register::W,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Operand {
    Reg(Register),
    Value(i32),
}

impl Operand {
    fn from_string(s: &str) -> Operand {
        match s.parse::<i32>() {
            Ok(val) => Operand::Value(val),
            Err(_) => Operand::Reg(Register::from_string(s))
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

fn parse_program(s: &String) -> Vec<Instruction> {
    s.split('\n')
        .filter_map(parse_instruction)
        .collect()
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let mut line = line.trim();
    if line.contains('#') {
        line = line.split_once('#').unwrap().0;
    }
    if line.is_empty() {
        return None;
    }
    let parts: Vec<_> = line.split(' ').collect();
    match parts[..] {
        ["inp", c] => Some(Instruction::Inp(Register::from_string(c))),
        ["add", a, b] => Some(Instruction::Add(Register::from_string(a), Operand::from_string(b))),
        ["mul", a, b] => Some(Instruction::Mul(Register::from_string(a), Operand::from_string(b))),
        ["div", a, b] => Some(Instruction::Div(Register::from_string(a), Operand::from_string(b))),
        ["mod", a, b] => Some(Instruction::Mod(Register::from_string(a), Operand::from_string(b))),
        ["eql", a, b] => Some(Instruction::Eql(Register::from_string(a), Operand::from_string(b))),
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/24.txt");
        b.iter(|| solve(&input.to_string()));
    }

    // #[test]
    // fn sample() {
    //     let data = "TODO".to_string();
    //     let (p1, p2) = solve(&data);
    //     assert_eq!(p1.as_deref(), Some("TODO").as_deref());
    //     assert_eq!(p2.as_deref(), Some("TODO").as_deref());
    // }
}

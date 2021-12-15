use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/TODO.txt");
        b.iter(|| black_box(solve(&input.to_string())));
    }

    #[test]
    fn sample() {
        let data = "TODO".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("TODO").as_deref());
        assert_eq!(p2.as_deref(), Some("TODO").as_deref());
    }
}

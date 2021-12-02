use crate::common::*;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let data = "TODO".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("TODO").as_deref());
        assert_eq!(p2.as_deref(), Some("TODO").as_deref());
    }
}

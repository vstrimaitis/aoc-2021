pub trait Solver {
    fn solve(&self, input: &String) -> (Option<String>, Option<String>);
}

use std::str;

pub type Solver = fn(&String) -> (Option<String>, Option<String>);

pub fn get_lines<'a>(input: &'a String) -> impl Iterator<Item = &'a str> {
    input.split('\n')
}

pub fn get_nonempty_lines<'a>(input: &'a String) -> impl Iterator<Item = &'a str> {
    get_lines(input).filter(|l| l.len() > 0)
}

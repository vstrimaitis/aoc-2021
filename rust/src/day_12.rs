use crate::common::*;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum CaveType {
    Start,
    End,
    Big,
    Small,
}

impl CaveType {
    fn from_name(name: &str) -> CaveType {
        match name {
            "start" => CaveType::Start,
            "end" => CaveType::End,
            name if name.to_uppercase() == name => CaveType::Big,
            name if name.to_lowercase() == name => CaveType::Small,
            _ => panic!("Invalid cave name")
        }
    }
}

#[derive(Clone, Debug)]
struct Cave {
    index: usize,
    kind: CaveType,
}

impl Cave {
    fn new(name: &str, index: usize) -> Cave {
        Cave{
            index: index,
            kind: CaveType::from_name(name)
        }
    }
}

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let edges: Vec<(&str, &str)> = get_nonempty_lines(input)
        .map(|s| s.split_once('-').expect("Failed to parse input"))
        .collect();
    
    let adj = build_graph(&edges);
    let mut path = Vec::new();
    let ans1 = dfs(&adj, 0, adj.len()-1, &mut path);
    let ans2 = dfs2(&adj, 0, adj.len()-1, &mut path);

    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn dfs(adj: &Vec<Vec<Cave>>, u: usize, end_index: usize, curr_path: &mut Vec<usize>) -> i32 {
    if u == end_index {
        return 1;
    }
    let mut ans = 0;
    curr_path.push(u);
    for v in &adj[u] {
        ans += match v {
            Cave{index: 0, ..} => 0,
            Cave{kind: CaveType::Small, ..} if curr_path.contains(&v.index) => 0,
            _ => dfs(adj, v.index, end_index, curr_path),
        };
    }
    curr_path.pop();
    ans
}

fn dfs2(adj: &Vec<Vec<Cave>>, u: usize, end_index: usize, curr_path: &mut Vec<usize>) -> i32 {
    if u == end_index {
        return 1;
    }
    let mut ans = 0;
    curr_path.push(u);
    for v in &adj[u] {
        ans += match v {
            Cave{index: 0, ..} => 0,
            Cave{kind: CaveType::Small, ..} if curr_path.contains(&v.index) => dfs(adj, v.index, end_index, curr_path),
            _ => dfs2(adj, v.index, end_index, curr_path),
        };
    }
    curr_path.pop();
    ans
}

fn build_graph(edges: &Vec<(&str, &str)>) -> Vec<Vec<Cave>> {
    let node_names: Vec<&str> = edges.iter()
        .flat_map(|&(u, v)| [u, v])
        .filter(|&s| s != "start" && s != "end")
        .unique()
        .collect();
    let mut name_to_index: HashMap<&str, usize> = HashMap::new();
    name_to_index.insert("start", 0);
    for (i, name) in node_names.iter().enumerate() {
        name_to_index.insert(name, i+1);
    }
    name_to_index.insert("end", name_to_index.values().max().unwrap() + 1);

    let mut adj = Vec::new();
    adj.resize(name_to_index.len(), vec![]);
    for (u, v) in edges {
        let u_idx = name_to_index[u];
        let v_idx = name_to_index[v];
        adj[u_idx].push(Cave::new(v, v_idx));
        adj[v_idx].push(Cave::new(u, u_idx));
    }

    adj
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        let input = include_str!("../../inputs/12.txt");
        b.iter(|| solve(&input.to_string()));
    }

    #[test]
    fn sample1() {
        let data = "start-A
start-b
A-c
A-b
b-d
A-end
b-end".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("10").as_deref());
        assert_eq!(p2.as_deref(), Some("36").as_deref());
    }

    #[test]
    fn sample2() {
        let data = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("19").as_deref());
        assert_eq!(p2.as_deref(), Some("103").as_deref());
    }

    #[test]
    fn sample3() {
        let data = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("226").as_deref());
        assert_eq!(p2.as_deref(), Some("3509").as_deref());
    }
}

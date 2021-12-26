use crate::common::*;
use std::collections::HashMap;

pub fn solve(input: &String) -> (Option<String>, Option<String>) {
    let (algo, board) = input.split_once("\n\n").expect("Failed to parse input");
    let algo: Vec<u8> = algo.chars().map(|c| (c == '#') as u8).collect();
    let coords: HashMap<(i16, i16), u16> = board.split("\n")
        .enumerate()
        .flat_map(|(i, r)| r.chars().enumerate().map(|(j, c)| ((i as i16, j as i16), (c == '#') as u16)).collect::<Vec<((i16, i16), u16)>>())
        .collect();

    let ans1 = run(&coords, &algo, 2);
    let ans2 = run(&coords, &algo, 50);
    
    (Some(ans1.to_string()), Some(ans2.to_string()))
}

fn run(lights: &HashMap<(i16, i16), u16>, algo: &Vec<u8>, n_times: usize) -> u16 {
    let mut lights = lights.clone();
    let outer_pixels = if algo.first().unwrap() == &1 && algo.last().unwrap() == &0 {
        [0, 1]
    } else {
        [0, 0]
    };
    for i in 0..n_times {
        lights = enhance(&lights, algo, outer_pixels[i % 2]);
    }
    lights.values().sum()
}

fn enhance(lights: &HashMap<(i16, i16), u16>, algo: &Vec<u8>, outer: u8) -> HashMap<(i16, i16), u16> {
    let rows: Vec<i16> = lights.keys().map(|&(i, _)| i).collect();
    let cols: Vec<i16> = lights.keys().map(|&(_, j)| j).collect();

    let min_row = *rows.iter().min().unwrap();
    let max_row = *rows.iter().max().unwrap();
    let min_col = *cols.iter().min().unwrap();
    let max_col = *cols.iter().max().unwrap();

    let mut new_lights = HashMap::new();
    for i in min_row-1..=max_row+1 {
        for j in min_col-1..=max_col+1 {
            let mut idx = 0u16;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let c = *lights.get(&(i+di, j+dj)).unwrap_or(&(outer as u16));
                    idx = 2*idx + c as u16;
                }
            }
            new_lights.insert((i, j), algo[idx as usize] as u16);
        }
    }
    new_lights
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    // #[bench]
    // fn bench_solve(b: &mut Bencher) {
    //     let input = include_str!("../../inputs/20.txt");
    //     b.iter(|| solve(&input.to_string()));
    // }

    #[test]
    fn sample() {
        let data = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###".to_string();
        let (p1, p2) = solve(&data);
        assert_eq!(p1.as_deref(), Some("35").as_deref());
        assert_eq!(p2.as_deref(), Some("3351").as_deref());
    }
}

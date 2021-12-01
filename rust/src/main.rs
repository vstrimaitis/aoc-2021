use std::fs;
use std::env;

mod common;
mod day_01;

use common::Solver;

fn get_solution(day: i32) -> Option<impl Solver> {
    match day {
        1 => Some(day_01::Solution),
        // _ => panic!("Solution for day {} is not registered!", day)
        _ => None,
    }
}

fn solve_day(day: i32) {
    let solution = get_solution(day);
    if solution.is_none() {
        // eprintln!("Solution for day {} is not registered - skipping...", day);
        return;
    }
    println!("Solving day {}...", day);
    let input_path = format!("../inputs/{:02}.txt", day);
    let input = fs::read_to_string(input_path).expect("Failed to read file");
    let (p1, p2) = solution.unwrap().solve(&input);
    if p1.is_some() {
        println!("    Part 1: {}", p1.unwrap());
    }
    if p2.is_some() {
        println!("    Part 2: {}", p2.unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let days = if args.len() == 1 {
        1..25
    } else {
        let day: i32 = args[1].parse::<i32>().expect("Day number must be an integer from 1 to 25");
        day..day+1
    };

    for day in days {
        solve_day(day)
    }
}

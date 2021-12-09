use std::env;
use std::fs;
use std::time::Instant;

mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
// !include modules

use common::Solver;

fn get_solution(day: i32) -> Option<Solver> {
    match day {
        1 => Some(day_01::solve),
        2 => Some(day_02::solve),
        3 => Some(day_03::solve),
        4 => Some(day_04::solve),
        5 => Some(day_05::solve),
        6 => Some(day_06::solve),
        7 => Some(day_07::solve),
        8 => Some(day_08::solve),
        // !include mapping
        _ => None,
    }
}

fn solve_day(day: i32) {
    let start_instant = Instant::now();

    let solution = get_solution(day);
    if solution.is_none() {
        return;
    }
    println!("Solving day {}...", day);
    let input_path = format!("../inputs/{:02}.txt", day);

    let mut now = Instant::now();
    let input = fs::read_to_string(input_path).expect("Failed to read file");
    let read_duration = now.elapsed();
    now = Instant::now();
    let (p1, p2) = solution.unwrap()(&input);
    let solve_duration = now.elapsed();

    if p1.is_some() {
        println!("    Part 1: {}", p1.unwrap());
    }
    if p2.is_some() {
        println!("    Part 2: {}", p2.unwrap());
    }
    let full_duration = start_instant.elapsed();
    println!("    > Input read duration: {:?}", read_duration);
    println!("    > Solve duration: {:?}", solve_duration);
    println!("    > Full duration for day {}: {:?}", day, full_duration);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let days = if args.len() == 1 {
        1..25
    } else {
        let day: i32 = args[1]
            .parse::<i32>()
            .expect("Day number must be an integer from 1 to 25");
        day..day + 1
    };

    let now = Instant::now();
    for day in days {
        solve_day(day)
    }
    let duration = now.elapsed();
    println!("Full duration: {:?}", duration);
}

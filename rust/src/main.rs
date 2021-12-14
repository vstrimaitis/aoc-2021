use std::env;
use std::fs;
use std::time::Instant;

mod ocr;
mod reporting;
mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
// !include modules

use common::Solver;
use reporting::{Report, ReportEntry};

fn get_solution(day: u32) -> Option<Solver> {
    match day {
        1 => Some(day_01::solve),
        2 => Some(day_02::solve),
        3 => Some(day_03::solve),
        4 => Some(day_04::solve),
        5 => Some(day_05::solve),
        6 => Some(day_06::solve),
        7 => Some(day_07::solve),
        8 => Some(day_08::solve),
        9 => Some(day_09::solve),
        10 => Some(day_10::solve),
        11 => Some(day_11::solve),
        12 => Some(day_12::solve),
        13 => Some(day_13::solve),
        14 => Some(day_14::solve),
        // !include mapping
        _ => None,
    }
}

fn solve_day(day: u32, report: &mut Report) {
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
    let full_duration = start_instant.elapsed();

    println!("Final report:");
    report.add_entry(ReportEntry {
        day,
        answer_1: p1,
        answer_2: p2,
        read_duration,
        solve_duration,
        full_duration,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let days = if args.len() == 1 {
        1..25
    } else {
        let day: u32 = args[1]
            .parse::<u32>()
            .expect("Day number must be an integer from 1 to 25");
        day..day + 1
    };

    let mut report = Report::start();
    for day in days {
        solve_day(day, &mut report);
    }
    report.end();
    report.display();
}

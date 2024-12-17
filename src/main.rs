#![deny(clippy::pedantic)]
use std::time::Duration;

use clap::Parser;
pub mod days;
pub mod solution;
pub mod utils;

use days::get_day;
use solution::{Part, SolvedValue};

/// Advent of Code 2024 runner implemented in Rust.
///
/// This is not necessarily an optimal set of solutions, but it works.
/// If you're interested in how this works, the code is available here:
/// <https://github.com/Snapstromegon/rustvent-of-code-24>
#[derive(Parser, Debug)]
struct Args {
    /// Day to run - if not set, run all days
    #[arg(short, long)]
    day: Option<usize>,

    /// Use example input instead of challenge input
    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn run_part(day: usize, part: Part, example: bool) -> Result<(SolvedValue, Duration), String> {
    let input = utils::read_input(day, example, part.into());
    if let Some(solution) = get_day(day) {
        if let Some(input) = input {
            let start = std::time::Instant::now();
            let result = solution.run(&input, part);
            let duration = start.elapsed();
            if let Some(result) = result {
                Ok((result, duration))
            } else {
                Err(format!("Day {day}.{part} not implemented"))
            }
        } else {
            Err(format!(
                "No input for day {day}.{part}"
            ))
        }
    } else {
        Err(format!("Day {day} not implemented"))
    }
}

fn run_day(day: usize, example: bool) {
    let res1 = run_part(day, Part::One, example);
    let res2 = run_part(day, Part::Two, example);

    print!("{day: >2} | ");
    match res1 {
        Ok((result, duration)) => print!("{result} {duration: >7.1?} | "),
        Err(e) => print!("{e: >24} | "),
    }
    match res2 {
        Ok((result, duration)) => println!("{result} {duration: >7.1?}"),
        Err(e) => println!("{e: >24}"),
    }
}

fn main() {
    let args = Args::parse();
    if let Some(day) = args.day {
        run_day(day, args.example);
    } else {
        let start = std::time::Instant::now();
        for day in 1..25 {
            if get_day(day).is_some() {
                run_day(day, args.example);
            }
        }
        let duration = start.elapsed();
        println!("Total time: {duration:.1?}");
    }
}

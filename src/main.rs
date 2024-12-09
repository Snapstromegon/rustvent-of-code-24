use clap::Parser;
pub mod days;
pub mod solution;
pub mod utils;

use days::*;
use solution::{Part, Solution};

#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    day: Option<usize>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn get_day(day: usize) -> Option<Box<dyn Solution>> {
    match day {
        0 => Some(Box::new(day00::Day)),
        1 => Some(Box::new(day01::Day)),
        2 => Some(Box::new(day02::Day)),
        3 => Some(Box::new(day03::Day)),
        4 => Some(Box::new(day04::Day)),
        5 => Some(Box::new(day05::Day)),
        6 => Some(Box::new(day06::Day)),
        7 => Some(Box::new(day07::Day)),
        8 => Some(Box::new(day08::Day)),
        9 => Some(Box::new(day09::Day)),
        _ => None,
    }
}

fn run_part(day: usize, part: Part, example: bool) {
    let input = utils::read_input(day, example, part.into());
    if let Some(solution) = get_day(day) {
        if let Some(input) = input {
            let start = std::time::Instant::now();
            let result = solution.run(&input, part);
            let duration = start.elapsed();
            if let Some(result) = result {
                println!("Part {}: {} - {:?}", part, result, duration);
            } else {
                println!("Part {}: not implemented for day {day}", part);
            }
        } else {
            println!(
                "Part {}: no input found for day {day} (example: {example})",
                part
            );
        }
    } else {
        println!("Day {} not implemented", day);
    }
}

fn run_day(day: usize, example: bool) {
    println!("Day {}:", day);
    run_part(day, Part::One, example);
    run_part(day, Part::Two, example);
}

fn main() {
    let args = Args::parse();
    if let Some(day) = args.day {
        run_day(day, args.example);
    } else {
        for day in 1..25 {
            if get_day(day).is_some() {
                run_day(day, args.example);
            }
        }
    }
}

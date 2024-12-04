use clap::Parser;
pub mod days;
pub mod solution;
pub mod utils;

use solution::Solution;
use days::*;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    day: usize,

    /// Number of times to greet
    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let example = args.example;
    let solution: Option<Box<dyn Solution>> = match day {
        0 => Some(Box::new(day00::Day)),
        1 => Some(Box::new(day01::Day)),
        2 => Some(Box::new(day02::Day)),
        3 => Some(Box::new(day03::Day)),
        4 => Some(Box::new(day04::Day)),
        _ => None,
    };
    if let Some(solution) = solution {
        println!("Day {}:", day);
        if let Some(input) = utils::read_input(day, example, 1) {
            let start = std::time::Instant::now();
            let result = solution.part1(&input);
            let duration = start.elapsed();
            if let Some(result) = result {
                println!("Part 1: {} (took {:?})", result, duration);
            } else {
                println!("Part 1: not implemented for day {day}");
            }
        } else {
            println!("Part 1: no input found for day {day} (example: {example})");
        }
        if let Some(input) = utils::read_input(day, example, 2) {
            let start = std::time::Instant::now();
            let result = solution.part2(&input);
            let duration = start.elapsed();
            if let Some(result) = result {
                println!("Part 2: {} (took {:?})", result, duration);
            } else {
                println!("Part 2: not implemented for day {day}");
            }
        } else {
            println!("Part 2: no input found for day {day} (example: {example})");
        }
    } else {
        println!("Day {} not implemented", day);
    }
}
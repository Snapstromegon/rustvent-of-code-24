#![deny(clippy::pedantic)]
use clap::Parser;
pub mod days;
pub mod solution;
pub mod utils;

use days::get_day;
use solution::Part;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    day: Option<usize>,

    /// Number of times to greet
    #[arg(short, long, default_value_t = false)]
    example: bool,
}

fn run_part(day: usize, part: Part, example: bool) {
    let input = utils::read_input(day, example, part.into());
    if let Some(solution) = get_day(day) {
        if let Some(input) = input {
            let start = std::time::Instant::now();
            let result = solution.run(&input, part);
            let duration = start.elapsed();
            if let Some(result) = result {
                println!("Part {part}: {result} - {duration:?}");
            } else {
                println!("Part {part}: not implemented for day {day}");
            }
        } else {
            println!("Part {part}: no input found for day {day} (example: {example})");
        }
    } else {
        println!("Day {day} not implemented");
    }
}

fn run_day(day: usize, example: bool) {
    println!("Day {day}:");
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

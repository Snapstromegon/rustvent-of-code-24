use crate::solution::Solution;

pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;

#[must_use]
pub fn get_day(day: usize) -> Option<Box<dyn Solution>> {
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
      10 => Some(Box::new(day10::Day)),
      11 => Some(Box::new(day11::Day)),
      12 => Some(Box::new(day12::Day)),
      13 => Some(Box::new(day13::Day)),
      14 => Some(Box::new(day14::Day)),
      15 => Some(Box::new(day15::Day)),
      16 => Some(Box::new(day16::Day)),
      17 => Some(Box::new(day17::Day)),
      18 => Some(Box::new(day18::Day)),
      19 => Some(Box::new(day19::Day)),
      20 => Some(Box::new(day20::Day)),
      _ => None,
  }
}

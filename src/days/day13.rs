#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
use std::str::FromStr;
extern crate nalgebra;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a_vec: (f64, f64),
    button_b_vec: (f64, f64),
    price_location: (f64, f64),
}

impl Machine {
    fn steps_to_price(&self) -> (usize, usize) {
        let equations = Matrix2::new(
            self.button_a_vec.0,
            self.button_b_vec.0,
            self.button_a_vec.1,
            self.button_b_vec.1,
        );
        let solutions = Vector2::new(self.price_location.0, self.price_location.1);
        let decomp = equations.lu();
        if let Some(res) = decomp.solve(&solutions) {
            let rounded_x = res.x.round();
            let rounded_y = res.y.round();
            if (rounded_x - res.x).abs() <= 0.001 && (rounded_y - res.y).abs() <= 0.001 {
                return (rounded_x as usize, rounded_y as usize);
            }
        }
        (0, 0)
    }

    fn min_tokens_price(&self) -> usize {
        let presses = self.steps_to_price();
        presses.0 * 3 + presses.1
    }
}

impl FromStr for Machine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let button_re = Regex::new(r"Button [AB]: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let button_a = button_re.captures(lines.next().unwrap()).unwrap();
        let button_b = button_re.captures(lines.next().unwrap()).unwrap();
        let price_re = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
        let price = price_re.captures(lines.next().unwrap()).unwrap();
        Ok(Machine {
            button_a_vec: (
                button_a["x"].parse().unwrap(),
                button_a["y"].parse().unwrap(),
            ),
            button_b_vec: (
                button_b["x"].parse().unwrap(),
                button_b["y"].parse().unwrap(),
            ),
            price_location: (price["x"].parse().unwrap(), price["y"].parse().unwrap()),
        })
    }
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|s| Machine::from_str(s).unwrap())
        .collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let machines = parse_machines(input);
        Some(
            machines
                .iter()
                .map(Machine::min_tokens_price)
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let machines = parse_machines(input);
        Some(
            machines
                .iter()
                .map(|machine| Machine {
                    price_location: (
                        machine.price_location.0 + 10_000_000_000_000.0,
                        machine.price_location.1 + 10_000_000_000_000.0,
                    ),
                    ..*machine
                })
                .map(|machine| machine.min_tokens_price())
                .sum::<usize>()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 13;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(480.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(39_996.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(875_318_608_908.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(73_267_584_326_867.into()));
    }
}

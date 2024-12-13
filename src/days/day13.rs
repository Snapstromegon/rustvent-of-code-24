use std::str::FromStr;

use regex::Regex;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a_vec: (usize, usize),
    button_b_vec: (usize, usize),
    price_location: (usize, usize),
}

impl Machine {
    fn vec_x_fac(&self) -> f64 {
        self.button_a_vec.0 as f64 / self.button_b_vec.0 as f64
    }
    fn vec_y_fac(&self) -> f64 {
        self.button_a_vec.1 as f64 / self.button_b_vec.1 as f64
    }

    fn vecs_same_direction(&self) -> bool {
        self.vec_x_fac() == self.vec_y_fac()
    }

    fn vecs_whole_multiple(&self) -> bool {
        self.vecs_same_direction() && self.vec_x_fac().fract() == 0.0
    }

    /// Price as a function of steps:
    /// price = 3a+b
    /// p = a*x + b*y
    /// p_x = a*x_1 + b*x_2
    /// p_y = a*y_1 + b*y_2
    fn steps_to_price(&self) -> (usize, usize) {
        for a in 1..=100 {
            for b in 1..=100 {
                let pos_x = self.button_a_vec.0 * a + self.button_b_vec.0 * b;
                let pos_y = self.button_a_vec.1 * a + self.button_b_vec.1 * b;
                if self.price_location == (pos_x, pos_y) {
                    return (a, b);
                }
            }
        }
        return (0, 0);
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
        let button_a_line = lines.next().unwrap();
        let button_b_line = lines.next().unwrap();
        let price_line = lines.next().unwrap();
        let button_re = Regex::new(r"Button [AB]: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let button_a = button_re.captures(button_a_line).unwrap();
        let button_b = button_re.captures(button_b_line).unwrap();
        let price_re = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
        let price = price_re.captures(price_line).unwrap();
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
    fn part1(&self, input: &str) -> Option<usize> {
        let machines = parse_machines(input);
        Some(
            machines
                .iter()
                .map(|machine| machine.min_tokens_price())
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        return None;
        let machines = parse_machines(input);
        Some(
            machines
                .iter()
                .map(|machine| Machine {
                    price_location: (machine.price_location.0 + 10_000_000_000_000, machine.price_location.1 + 10_000_000_000_000),
                    ..machine.clone()
                })
                .map(|machine| machine.min_tokens_price())
                .sum(),
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
        assert_eq!(Day.part1(&input), Some(480));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(39996));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}

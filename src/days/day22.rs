#![allow(clippy::cast_possible_wrap)]

use std::collections::VecDeque;

use rayon::prelude::*;

use crate::solution::{Solution, SolvedValue};

fn mix_and_prune(current: usize, value: usize) -> usize {
    (current ^ value) % 16_777_216
}

fn next_secret(current: usize) -> usize {
    let mut secret = current;
    secret = mix_and_prune(secret, secret * 64);
    secret = mix_and_prune(secret, secret / 32);
    secret = mix_and_prune(secret, secret * 2048);
    secret
}

fn get_price(secret: usize) -> usize {
    secret % 10
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn get_diff_index(x1: isize, x2: isize, x3: isize, x4: isize) -> usize {
    ((x1 + 10) * 20isize.pow(3) + (x2 + 10) * 20isize.pow(2) + (x3 + 10) * 20isize + (x4 + 10))
        .try_into()
        .unwrap()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        Some(
            parse_input(input)
                .into_par_iter()
                .map(|secret| (0..2000).fold(secret, |secret, _| next_secret(secret)))
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let start_secrets = parse_input(input);
        let mut max = 0;
        let mut global_diffs = vec![0usize; 20usize.pow(4)];

        for mut secret in start_secrets {
            let mut last_price = get_price(secret);
            let mut diffs = VecDeque::with_capacity(4);
            let mut seen = vec![false; 20usize.pow(4)];
            for _ in 0..2000 {
                secret = next_secret(secret);
                let current_price = get_price(secret);
                let price_diff = last_price as isize - current_price as isize;
                last_price = current_price;
                diffs.push_back(price_diff);
                if diffs.len() > 3 {
                    let key = get_diff_index(diffs[0], diffs[1], diffs[2], diffs[3]);
                    if !seen[key] {
                        seen[key] = true;
                        global_diffs[key] += current_price;
                        max = max.max(global_diffs[key]);
                    }
                    diffs.pop_front();
                }
            }
        }
        Some(max.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 22;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(37_327_623.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(15_613_157_363.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(23.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1784.into()));
    }
}

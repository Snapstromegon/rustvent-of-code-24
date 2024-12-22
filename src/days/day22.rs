use std::collections::{HashMap, HashSet, VecDeque};

use rayon::prelude::*;

use crate::solution::{Solution, SolvedValue};

fn mix_and_prune(current: usize, value: usize) -> usize {
    (current ^ value) % 16777216
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
        let inputs = parse_input(input);

        let mut global_diffs = HashMap::new();

        inputs.into_iter().for_each(|mut secret| {
            let mut last_price = get_price(secret);
            let mut diffs = VecDeque::with_capacity(4);
            let mut seen = HashSet::new();
            for _ in 0..2000 {
                secret = next_secret(secret);
                let current_price = get_price(secret);
                let price_diff = last_price as isize - current_price as isize;
                last_price = current_price;
                diffs.push_back(price_diff);
                if diffs.len() > 3 {
                    if !seen.contains(&diffs) {
                        seen.insert(diffs.clone());
                        global_diffs
                            .entry(diffs.clone())
                            .and_modify(|x| *x += current_price)
                            .or_insert(current_price);
                    }
                    diffs.pop_front();
                }
            }
        });

        Some(global_diffs.values().max().unwrap().clone().into())
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
    #[ignore = "takes too long"]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1784.into()));
    }
}

use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, Clone, Copy)]
struct Stone(usize);

impl Stone {
    fn blink(self, n: usize, cache: &Arc<RwLock<HashMap<usize, HashMap<usize, usize>>>>) -> usize {
        if n == 0 {
            1
        } else {
            if let Some(&result) = cache.read().unwrap().get(&self.0).and_then(|m| m.get(&n)) {
                return result;
            }
            let count = if self.0 == 0 {
                Stone(1).blink(n - 1, cache)
            } else if self.0.ilog10() % 2 == 1 {
                let factor = 10usize.pow((self.0.ilog10() + 1) / 2);
                let left = Stone(self.0 / factor);
                let right = Stone(self.0 - left.0 * factor);
                left.blink(n - 1, cache) + right.blink(n - 1, cache)
            } else {
                Stone(self.0 * 2024).blink(n - 1, cache)
            };
            cache
                .write()
                .unwrap()
                .entry(self.0)
                .or_default()
                .insert(n, count);
            count
        }
    }
}

fn parse_input(input: &str) -> Vec<Stone> {
    input
        .split_whitespace()
        .map(|line| Stone(line.parse().unwrap()))
        .collect()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let stones = parse_input(input);
        Some(
            stones
                .par_iter()
                .map(|stone| stone.blink(25, &Arc::new(RwLock::new(HashMap::new()))))
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let stones = parse_input(input);
        Some(
            stones
                .par_iter()
                .map(|stone| stone.blink(75, &Arc::new(RwLock::new(HashMap::new()))))
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

    const DAY: usize = 11;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(55_312.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(186_424.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(65_601_038_650_482.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(219_838_428_124_832.into()));
    }
}

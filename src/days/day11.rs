use std::collections::HashMap;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy)]
struct Stone(usize);

impl Stone {
    fn blink(&self, n: usize, cache: &mut HashMap<usize, HashMap<usize, usize>>) -> usize {
        if n == 0 {
            1
        } else {
            if let Some(&result) = cache.get(&self.0).and_then(|m| m.get(&n)) {
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
                .entry(self.0)
                .or_insert_with(HashMap::new)
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
    fn part1(&self, input: &str) -> Option<usize> {
        let stones = parse_input(input);
        let mut cache = HashMap::new();
        Some(stones.iter().map(|stone| stone.blink(25,&mut cache)).sum())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let stones = parse_input(input);
        let mut cache = HashMap::new();
        Some(stones.iter().map(|stone| stone.blink(75,&mut cache)).sum())
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
        assert_eq!(Day.part1(&input), Some(55312));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(186424));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(65601038650482));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(219838428124832));
    }
}

use std::collections::HashMap;

use crate::solution::Solution;

fn get_left_right_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let left_part = parts.next().unwrap().parse().unwrap();
        let right_part = parts.next().unwrap().parse().unwrap();
        left.push(left_part);
        right.push(right_part);
    }
    (left, right)
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (mut left, mut right) = get_left_right_lists(input);
        left.sort_unstable();
        right.sort_unstable();

        let total_distance: i64 = left.iter().zip(right).map(|(l, r)| (l - r).abs()).sum();
        Some(usize::try_from(total_distance).unwrap())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (left, right) = get_left_right_lists(input);

        let mut right_hash: HashMap<i64, i64> = HashMap::new();
        for r in right {
            *right_hash.entry(r).or_insert(0) += 1;
        }

        let result: i64 = left
            .iter()
            .map(|l| l * right_hash.get(l).unwrap_or(&0))
            .sum();
        Some(usize::try_from(result).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 1;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(11));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(2057374));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(31));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(23177084));
    }
}

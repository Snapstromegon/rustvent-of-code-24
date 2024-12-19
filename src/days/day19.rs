use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use rayon::prelude::*;

use crate::solution::{Solution, SolvedValue};

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let towels = lines.next().unwrap().split(", ").collect();
    lines.next();
    let designs = lines.collect();
    (towels, designs)
}

fn design_possible_count(
    design: &str,
    towels: &[&str],
    known_results: &Arc<RwLock<HashMap<String, usize>>>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&result) = known_results.read().unwrap().get(design) {
        return result;
    }
    let result = towels
        .iter()
        .filter(|&&towel| design.starts_with(towel))
        .map(|towel| design_possible_count(&design[towel.len()..], towels, known_results))
        .sum();
    if known_results.read().unwrap().get(design).is_none() {
        known_results
            .write()
            .unwrap()
            .insert(design.to_string(), result);
    }
    result
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let (towels, designs) = parse_input(input);
        let known_results = Arc::new(RwLock::new(HashMap::new()));
        let count = designs
            .par_iter()
            .filter(|design| design_possible_count(design, &towels, &known_results) > 0)
            .count();
        Some(count.into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let (towels, designs) = parse_input(input);
        let known_results = Arc::new(RwLock::new(HashMap::new()));
        let count: usize = designs
            .par_iter()
            .map(|design| design_possible_count(design, &towels, &known_results))
            .sum();
        Some(count.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 19;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(6.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(258.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(16.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(632_423_618_484_345.into()));
    }
}

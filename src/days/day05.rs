use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::solution::Solution;

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut parts = input.split("\n\n");
    let rules_part = parts.next().unwrap();
    let pages_part = parts.next().unwrap();

    let rules = rules_part
        .lines()
        .map(|line| {
            let mut line_parts = line.split("|");
            (
                line_parts.next().unwrap().parse().unwrap(),
                line_parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let pages = pages_part
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, pages)
}

fn get_rules_map(raw_rules: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut rules_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (before, after) in raw_rules {
        rules_map.entry(*before).or_default().push(*after);
    }
    rules_map
}

fn middle_page(pages: &[usize]) -> usize {
    pages[pages.len() / 2]
}

fn is_pages_sorted(pages: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut seen = HashSet::new();
    for page in pages {
        for must_be_after in rules.get(page).unwrap_or(&Vec::new()) {
            if seen.contains(must_be_after) {
                return false;
            }
        }
        seen.insert(page);
    }
    true
}

fn sorted_pages(pages: &[usize], rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut result = pages.to_vec();

    result.sort_by(|a, b| {
        if let Some(after_pages) = rules.get(a) {
            if after_pages.contains(b) {
                return Ordering::Less;
            }
        }
        if let Some(after_pages) = rules.get(b) {
            if after_pages.contains(a) {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });

    result
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (rules, pages_list) = parse_input(input);
        let rules_map = get_rules_map(&rules);

        Some(
            pages_list
                .iter()
                .filter(|pages| is_pages_sorted(pages, &rules_map))
                .map(|pages| middle_page(pages))
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (rules, pages_list) = parse_input(input);
        let rules_map = get_rules_map(&rules);

        Some(
            pages_list
                .iter()
                .filter(|pages| !is_pages_sorted(pages, &rules_map))
                .map(|pages| sorted_pages(pages, &rules_map))
                .map(|pages| middle_page(&pages))
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 5;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(143));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(5391));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(123));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(6142));
    }
}

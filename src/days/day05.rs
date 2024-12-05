use std::cmp::Ordering;

use crate::solution::Solution;

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut parts = input.split("\n\n");
    let rules_part = parts.next().unwrap();
    let pages_part = parts.next().unwrap();

    let mut rules: Vec<Vec<usize>> = Vec::with_capacity(100);
    rules.resize_with(100, Default::default);

    for rule in rules_part.lines() {
        let mut line_parts = rule.split("|");
        rules[line_parts.next().unwrap().parse::<usize>().unwrap()]
            .push(line_parts.next().unwrap().parse().unwrap());
    }

    let pages = pages_part
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, pages)
}

fn middle_page(pages: &[usize]) -> usize {
    pages[pages.len() / 2]
}

fn is_pages_sorted(pages: &[usize], rules: &[Vec<usize>]) -> bool {
    // we know that numbers are always two digits
    let mut seen = [false; 100];
    for page in pages {
        for must_be_after in &rules[*page] {
            if seen[*must_be_after] {
                return false;
            }
        }
        seen[*page] = true;
    }
    true
}

fn sorted_pages(pages: &[usize], rules: &[Vec<usize>]) -> Vec<usize> {
    let mut result = pages.to_vec();

    result.sort_unstable_by(|a, b| {
        if rules[*a].contains(b) {
            return Ordering::Less;
        }
        if rules[*b].contains(a) {
            return Ordering::Greater;
        }
        Ordering::Equal
    });

    result
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (rules, pages_list) = parse_input(input);

        Some(
            pages_list
                .iter()
                .filter(|pages| is_pages_sorted(pages, &rules))
                .map(|pages| middle_page(pages))
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (rules, pages_list) = parse_input(input);

        Some(
            pages_list
                .iter()
                .filter(|pages| !is_pages_sorted(pages, &rules))
                .map(|pages| sorted_pages(pages, &rules))
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

use crate::solution::Solution;
use rayon::prelude::*;

fn parse_equations(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (result_str, params) = line.split_once(": ").unwrap();
            (
                result_str.parse().unwrap(),
                params
                    .split_whitespace()
                    .map(|p| p.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn recursive_is_solvable(
    expected: usize,
    current: usize,
    params: &[usize],
    ops: &[Operator],
) -> bool {
    if params.is_empty() {
        expected == current
    } else {
        ops.iter().any(|op| {
            recursive_is_solvable(expected, op.apply(current, params[0]), &params[1..], ops)
        })
    }
}

#[derive(Clone, Debug, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => a * (10_usize).pow(b.ilog10() + 1) + b,
        }
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let ops = vec![Operator::Add, Operator::Multiply];
        Some(
            parse_equations(input)
                .par_iter()
                .filter(|eq| recursive_is_solvable(eq.0, eq.1[0], &eq.1[1..], &ops))
                .map(|eq| eq.0)
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let ops = vec![Operator::Add, Operator::Multiply, Operator::Concat];
        Some(
            parse_equations(input)
                .par_iter()
                .filter(|eq| recursive_is_solvable(eq.0, eq.1[0], &eq.1[1..], &ops))
                .map(|eq| eq.0)
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 7;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(3749));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(2_299_996_598_890));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(11387));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(362_646_859_298_554));
    }
}

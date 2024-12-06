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

fn is_equations_solvable((result, params): &(usize, Vec<usize>), valid_ops: &[Operator]) -> bool {
    let mut base_operators = Vec::with_capacity(params.len() - 1);
    base_operators.fill(valid_ops[0]);

    let ops_count = valid_ops.len();
    let total_tries = ops_count.pow(params.len() as u32 - 1);
    let pre_calced_ops_pow: Vec<usize> = params
        .iter()
        .skip(1)
        .enumerate()
        .map(|(j, _)| ops_count.pow(j as u32))
        .collect();

    for i in 0..=total_tries {
        let compare = params
            .iter()
            .skip(1)
            .enumerate()
            .fold(params[0], |acc, (j, p)| {
                let op_index = (i / pre_calced_ops_pow[j]) % ops_count;
                valid_ops[op_index].apply(acc, *p)
            });
        if compare == *result {
            return true;
        }
    }
    false
}

#[derive(Clone, Debug, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => a * (10 as usize).pow(b.ilog10() + 1) + b,
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
                .filter(|eq| is_equations_solvable(*eq, &ops))
                .map(|eq| eq.0)
                .sum(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let ops = vec![Operator::Add, Operator::Multiply, Operator::Concat];
        Some(
            parse_equations(input)
                .par_iter()
                .filter(|eq| is_equations_solvable(*eq, &ops))
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
        assert_eq!(Day.part1(&input), Some(2299996598890));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(11387));
    }
    #[test]
    #[ignore = "takes too long"]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(362646859298554));
    }
}

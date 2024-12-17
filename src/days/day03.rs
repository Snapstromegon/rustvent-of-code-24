use crate::solution::{Solution, SolvedValue};
use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};
pub struct Day;

fn nom_multiplication(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(
        tag("mul("),
        separated_pair(
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::u32,
        ),
        tag(")"),
    )(input)
}

fn nom_muls(input: &str) -> usize {
    let (_, muls) = many0(many_till(anychar, nom_multiplication))(input).unwrap();

    muls.iter().map(|(_, (a, b))| (a * b) as usize).sum()
}

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        Some(nom_muls(input).into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let do_split = input.split("do()");
        let mut sum = 0;
        for part in do_split {
            let enabled = part.split("don't()").next().unwrap();
            sum += nom_muls(enabled);
        }
        Some(sum.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 3;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(161.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(190_604_937.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(48.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(82_857_512.into()));
    }
}

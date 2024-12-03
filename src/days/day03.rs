use crate::solution::Solution;
use regex::Regex;

pub struct Day;

fn run_muls(input: &str) -> usize {
    let re = Regex::new(r"mul\((?P<fac1>\d{1,3}),(?P<fac2>\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let fac1 = cap["fac1"].parse::<usize>().unwrap();
            let fac2 = cap["fac2"].parse::<usize>().unwrap();
            fac1 * fac2
        })
        .sum()
}

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        Some(run_muls(input))
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let do_split = input.split("do()");
        let mut sum = 0;
        for part in do_split {
            let enabled = part.split("don't()").next().unwrap();
            sum += run_muls(enabled);
        }
        Some(sum)
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
        assert_eq!(Day.part1(&input), Some(161));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(190604937));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(48));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(82857512));
    }
}

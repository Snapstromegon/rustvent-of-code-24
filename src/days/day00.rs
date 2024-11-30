use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, _input: &str) -> Option<usize> {
        None
    }

    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    #[test]
    fn test_part1_example() {
        let input = read_input(0, true, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(0, false, 1).unwrap();
        assert_eq!(Day.part1(&input), None);
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(0, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(0, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}

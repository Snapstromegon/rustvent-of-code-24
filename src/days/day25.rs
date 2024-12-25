use crate::solution::{Solution, SolvedValue};

#[derive(Debug)]
struct Lock {
    pins: [usize; 5],
}

impl Lock {
    fn fits(&self, key: &Key) -> bool {
        for i in 0..5 {
            if self.pins[i] + key.pins[i] > 5 {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Key {
    pins: [usize; 5],
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let items = input.split("\n\n");
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for item in items {
        let lines = item.lines().collect::<Vec<_>>();
        if lines[0].starts_with('#') {
            let mut lock = Lock {
                pins: [0, 0, 0, 0, 0],
            };
            for (row, line) in lines.iter().enumerate() {
                for (col, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock.pins[col] = row;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = Key {
                pins: [0, 0, 0, 0, 0],
            };
            for (row, line) in lines.iter().rev().enumerate() {
                for (col, c) in line.chars().enumerate() {
                    if c == '#' {
                        key.pins[col] = row;
                    }
                }
            }
            keys.push(key);
        }
    }

    (locks, keys)
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let (locks, keys) = parse_input(input);
        Some(
            locks
                .iter()
                .map(|lock| keys.iter().filter(|key| lock.fits(key)).count())
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, _input: &str) -> Option<SolvedValue> {
        Some("Day 25.2 was a gift!".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 25;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(3.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(3264.into()));
    }
}

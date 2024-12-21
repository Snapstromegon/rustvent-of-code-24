use crate::solution::{Solution, SolvedValue};

type Position = (usize, usize);

fn parse_input(input: &str) -> Vec<(usize, Vec<Position>)> {
    input
        .lines()
        .map(|line| {
            (
                line.split_once("A").unwrap().0.parse().unwrap(),
                line.chars()
                    .map(|c| match c {
                        'A' => (3, 2),
                        '0' => (3, 1),
                        '1' => (2, 0),
                        '2' => (2, 1),
                        '3' => (2, 2),
                        '4' => (1, 0),
                        '5' => (1, 1),
                        '6' => (1, 2),
                        '7' => (0, 0),
                        '8' => (0, 1),
                        '9' => (0, 2),
                        _ => panic!("Invalid input"),
                    })
                    .collect(),
            )
        })
        .collect()
}

fn get_directional_keyboard_inputs_for(
    key: &[Position],
    mut pos: Position,
    forbidden: Position,
) -> Vec<Position> {
    let mut res = vec![];
    for (ty, tx) in key {
        // Special case forbidden
        if pos.0 == forbidden.0 && *ty != forbidden.0 {
            if *ty > pos.0 {
                for _ in 0..(ty - pos.0) {
                    res.push((1, 1));
                }
            } else if *ty < pos.0 {
                for _ in 0..(pos.0 - ty) {
                    res.push((0, 1));
                }
            }
            pos = (*ty, pos.1);
        }

        if pos.1 == forbidden.1 && *tx != forbidden.1 {
            if *tx > pos.1 {
                for _ in 0..(tx - pos.1) {
                    res.push((1, 2));
                }
            } else if *tx < pos.1 {
                for _ in 0..(pos.1 - tx) {
                    res.push((1, 0));
                }
            }
            pos = (pos.0, *tx);
        }

        if *tx < pos.1 {
            for _ in 0..(pos.1 - tx) {
                res.push((1, 0));
            }
        }
        if *ty > pos.0 {
            for _ in 0..(ty - pos.0) {
                res.push((1, 1));
            }
        }
        if *ty < pos.0 {
            for _ in 0..(pos.0 - ty) {
                res.push((0, 1));
            }
        }
        if *tx > pos.1 {
            for _ in 0..(tx - pos.1) {
                res.push((1, 2));
            }
        }
        res.push((0, 2));
        pos = (*ty, *tx);
    }
    res
}

fn print_dir_input(input: &[Position]) {
    for pos in input {
        print!(
            "{}",
            match pos {
                (0, 2) => 'A',
                (0, 1) => '^',
                (1, 0) => '<',
                (1, 1) => 'v',
                (1, 2) => '>',
                _ => panic!("Invalid input"),
            }
        );
    }
    println!();
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let keys = parse_input(input);
        Some(
            keys.iter()
                .map(|(complexity, key)| {
                    println!("########################");
                    println!("Key: {key:?}");
                    let numpad_input = get_directional_keyboard_inputs_for(key, (3, 2), (3, 0));
                    print!("Numpad input: ");
                    print_dir_input(&numpad_input);
                    let dir_bot1_input =
                        get_directional_keyboard_inputs_for(&numpad_input, (0, 2), (0, 0));
                    print!("Dir bot 1 input: ");
                    print_dir_input(&dir_bot1_input);
                    let dir_bot2_input =
                        get_directional_keyboard_inputs_for(&dir_bot1_input, (0, 2), (0, 0));
                    print!("Dir bot 2 input: ");
                    print_dir_input(&dir_bot2_input);
                    println!("Complexity: {}, len: {}", complexity, dir_bot2_input.len());
                    complexity * dir_bot2_input.len()
                })
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, _input: &str) -> Option<SolvedValue> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 21;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(126_384.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(215_374.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}

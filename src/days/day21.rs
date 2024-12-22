use std::collections::HashMap;

use crate::solution::{Solution, SolvedValue};

type Position = (usize, usize);

fn parse_input(input: &str) -> Vec<(usize, Vec<Position>)> {
    input
        .lines()
        .map(|line| {
            (
                line.split_once('A').unwrap().0.parse().unwrap(),
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
    start_pos: Position,
    forbidden: Position,
    recurse_count: usize,
    cache: &mut HashMap<(Vec<Position>, Position, Position, usize), usize>,
) -> usize {
    if recurse_count == 0 {
        return key.len();
    }
    if let Some(&res) = cache.get(&(key.to_vec(), start_pos, forbidden, recurse_count)) {
        return res;
    }
    let mut pos = start_pos;
    let mut res = 0;
    for (ty, tx) in key {
        let mut key_res = vec![];
        let v_diff = (0..ty.abs_diff(pos.0))
            .map(|_| if *ty > pos.0 { (1, 1) } else { (0, 1) })
            .collect::<Vec<_>>();
        let h_diff = (0..tx.abs_diff(pos.1))
            .map(|_| if *tx > pos.1 { (1, 2) } else { (1, 0) })
            .collect::<Vec<_>>();
        // Special case forbidden
        if ((pos.0 == forbidden.0 || *ty == forbidden.0) && pos.0 != *ty)
            && ((pos.1 == forbidden.1 || *tx == forbidden.1) && pos.1 != *tx)
        {
            if *tx < pos.1 {
                key_res.extend(v_diff);
                key_res.extend(h_diff);
            } else {
                key_res.extend(h_diff);
                key_res.extend(v_diff);
            }
        } else {
            if *tx < pos.1 {
                key_res.extend_from_slice(&h_diff);
            }
            if *ty != pos.0 {
                key_res.extend(v_diff);
            }
            if *tx > pos.1 {
                key_res.extend(h_diff);
            }
        }

        key_res.push((0, 2));
        res +=
            get_directional_keyboard_inputs_for(&key_res, (0, 2), (0, 0), recurse_count - 1, cache);
        pos = (*ty, *tx);
    }
    cache.insert((key.to_vec(), pos, forbidden, recurse_count), res);
    res
}

fn get_minimal_combination_for(key: &[Position], dir_bot_count: usize) -> usize {
    get_directional_keyboard_inputs_for(key, (3, 2), (3, 0), dir_bot_count + 1, &mut HashMap::new())
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let keys = parse_input(input);
        Some(
            keys.iter()
                .map(|(complexity, key)| complexity * get_minimal_combination_for(key, 2))
                .sum::<usize>()
                .into(),
        )
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let keys = parse_input(input);
        Some(
            keys.iter()
                .map(|(complexity, key)| complexity * get_minimal_combination_for(key, 25))
                .sum::<usize>()
                .into(),
        )
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
        assert_eq!(Day.part2(&input), Some(154_115_708_116_294.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(260_586_897_262_600.into()));
    }
}

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]
use std::{collections::VecDeque, str::FromStr};

use rayon::prelude::*;

use crate::solution::{Solution, SolvedValue};

type Position = (usize, usize);

#[derive(Debug)]
struct Maze {
    drop_list: Vec<Position>,
    size: usize,
}

impl Maze {
    fn get_corrupted_after_bytes(&self, bytes: usize) -> Vec<Vec<bool>> {
        let mut distances = vec![vec![false; self.size + 1]; self.size + 1];
        for block in self.drop_list.iter().take(bytes) {
            distances[block.1][block.0] = true;
        }
        distances
    }

    fn find_distance_to_end(&self, bytes: usize) -> Option<usize> {
        let corrupted = self.get_corrupted_after_bytes(bytes);
        let mut distances = vec![vec![None; self.size + 1]; self.size + 1];

        let mut queue: VecDeque<Position> = VecDeque::with_capacity(self.size * self.size);
        queue.push_back((0, 0));
        distances[0][0] = Some(0);

        while let Some((x, y)) = queue.pop_front() {
            if x == self.size && y == self.size {
                return Some(distances[y][x].unwrap());
            }

            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;

                if new_x < 0
                    || new_x > self.size as isize
                    || new_y < 0
                    || new_y > self.size as isize
                {
                    continue;
                }

                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if corrupted[new_y][new_x] {
                    continue;
                }

                if distances[new_y][new_x].is_none() {
                    distances[new_y][new_x] = Some(distances[y][x].unwrap() + 1);
                    queue.push_back((new_x, new_y));
                }
            }
        }

        None
    }
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let drop_list: Vec<Position> = s
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect();
        Ok(Maze {
            size: if drop_list.iter().any(|x| x.0 > 6 || x.1 > 6) {
                70
            } else {
                6
            },
            drop_list,
        })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let maze: Maze = input.parse().unwrap();
        let bytes_to_run = if maze.size == 6 { 12 } else { 1024 };
        maze.find_distance_to_end(bytes_to_run)
            .map(SolvedValue::from)
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let maze: Maze = input.parse().unwrap();

        (1..maze.drop_list.len())
            .par_bridge()
            .filter(|i| maze.find_distance_to_end(*i).is_none())
            .min()
            .map(|i| {
                SolvedValue::from(format!(
                    "{},{}",
                    maze.drop_list[i - 1].0,
                    maze.drop_list[i - 1].1
                ))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 18;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(22.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(370.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some("6,1".into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some("65,6".into()));
    }
}

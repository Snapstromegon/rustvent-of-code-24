#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use std::{
    ops::Neg,
    str::FromStr,
};

use rayon::prelude::*;

use crate::solution::{Solution, SolvedValue};

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Wall,
    Path(Option<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Maze {
    tiles: Vec<Vec<Block>>,
    start: Position,
    end: Position,
    path: Vec<Position>,
}

impl Maze {
    fn next_position(&self, current: Position) -> Position {
        [
            (current.0 + 1, current.1),
            (current.0, current.1 + 1),
            (current.0 - 1, current.1),
            (current.0, current.1 - 1),
        ]
        .into_iter()
        .find(|(x, y)| self.tiles[*x][*y] == Block::Path(None))
        .unwrap()
    }

    fn get_cheat_neighbors(
        &self,
        current: Position,
        max_distance: isize,
        min_save: usize,
    ) -> usize {
        let Block::Path(Some(current_distance)) = self.tiles[current.0][current.1] else {
            unreachable!("Should not be a wall");
        };

        let mut candidates = 0;

        for i in max_distance.neg()..=max_distance {
            for j in max_distance.neg()..=max_distance {
                if i.abs() + j.abs() > max_distance {
                    continue;
                }
                let y = current.0 as isize + i;
                let x = current.1 as isize + j;
                if y < 0 || x < 0 {
                    continue;
                }
                let y = y as usize;
                let x = x as usize;
                if y >= self.tiles.len() || x >= self.tiles[0].len() {
                    continue;
                }
                if let Block::Path(Some(d)) = self.tiles[y][x] {
                    if d.saturating_sub(current_distance).saturating_sub(i.unsigned_abs()).saturating_sub(j.unsigned_abs()) >= min_save
                    {
                        candidates+=1;
                    }
                }
            }
        }

        candidates
    }

    fn calculate_distances(&mut self) {
        let mut distance = 0;
        let mut current = self.start;
        // We know only one path exists
        while current != self.end {
            self.tiles[current.0][current.1] = Block::Path(Some(distance));
            self.path.push(current);
            distance += 1;
            current = self.next_position(current);
        }
        self.tiles[current.0][current.1] = Block::Path(Some(distance));
        self.path.push(current);
    }

    fn find_shortcuts_count(&self, max_distance: isize, min_save: usize) -> usize {
        self.path
            .par_iter()
            .map(|entry| self.get_cheat_neighbors(*entry, max_distance, min_save))
            .sum()
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut tiles = Vec::new();
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        row.push(Block::Wall);
                    }
                    '.' => {
                        row.push(Block::Path(None));
                    }
                    'S' => {
                        row.push(Block::Path(None));
                        start = (y, x);
                    }
                    'E' => {
                        row.push(Block::Path(None));
                        end = (y, x);
                    }
                    _ => {
                        unreachable!("Should not be in input");
                    }
                }
            }
            tiles.push(row);
        }

        Ok(Maze {
            tiles,
            start,
            end,
            path: Vec::new(),
        })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let mut maze: Maze = input.parse().unwrap();
        maze.calculate_distances();
        Some(maze.find_shortcuts_count(2, 100).into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let mut maze: Maze = input.parse().unwrap();
        maze.calculate_distances();
        Some(maze.find_shortcuts_count(20, 100).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 20;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(0.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1411.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(0.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1_010_263.into()));
    }
}

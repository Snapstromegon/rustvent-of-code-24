use std::{collections::HashSet, str::FromStr};

use crate::solution::Solution;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: isize,
    col: isize,
}

impl Position {
    fn add(&self, vector: (isize, isize)) -> Position {
        Position {
            row: self.row + vector.0,
            col: self.col + vector.1,
        }
    }
}

#[derive(Debug)]
struct Field {
    guard_pos: Position,
    guard_dir: Direction,
    obstacles: HashSet<Position>,
    visited: HashSet<Position>,
    rows: usize,
    cols: usize,
}

impl Field {
    fn next_guard_pos(&self) -> Position {
        self.guard_pos.add(self.guard_dir.vector())
    }

    fn is_step_possible(&self) -> bool {
        !self.obstacles.contains(&self.next_guard_pos())
    }

    fn guard_step(&mut self) {
        while !self.is_step_possible() {
            self.guard_dir = self.guard_dir.turn_right();
        }
        self.visited.insert(self.guard_pos);
        self.guard_pos = self.next_guard_pos();
    }

    fn is_guard_inside(&self) -> bool {
        (0..self.rows as isize).contains(&self.guard_pos.row)
            && (0..self.cols as isize).contains(&self.guard_pos.col)
    }

    fn simulate_to_exit(&mut self) {
        while self.is_guard_inside() {
            self.guard_step();
        }
    }
}

impl FromStr for Field {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maybe_guard_pos = None;
        let mut obstacles = HashSet::new();
        let mut rows = 0;
        let mut cols = 0;
        for (row, line) in s.lines().enumerate() {
            rows = row;
            for (col, char) in line.chars().enumerate() {
                cols = col;
                match char {
                    '^' => {
                        maybe_guard_pos = Some(Position {
                            row: row as isize,
                            col: col as isize,
                        })
                    }
                    '#' => {
                        obstacles.insert(Position {
                            row: row as isize,
                            col: col as isize,
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(Field {
            guard_pos: maybe_guard_pos.ok_or("No Guard found")?,
            guard_dir: Direction::Up,
            obstacles,
            visited: HashSet::new(),
            rows: rows + 1,
            cols: cols + 1,
        })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut field: Field = input.parse().expect("Field not parsable");
        field.simulate_to_exit();
        Some(field.visited.len())
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

    const DAY: usize = 6;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(41));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(4789));
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

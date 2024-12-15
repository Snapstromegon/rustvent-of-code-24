#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
use std::{fmt::Display, ops::Add, str::FromStr};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Robot,
    Box,
    Wall,
    Empty,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            State::Robot => '@',
            State::Box => 'O',
            State::Wall => '#',
            State::Empty => '.',
        };
        write!(f, "{c}")
    }
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.chars().next().unwrap();
        let state = match c {
            '@' => State::Robot,
            'O' => State::Box,
            '#' => State::Wall,
            '.' => State::Empty,
            _ => return Err(format!("Invalid state: {c}")),
        };
        Ok(state)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "^" => Direction::Up,
            "v" => Direction::Down,
            "<" => Direction::Left,
            ">" => Direction::Right,
            _ => return Err(format!("Invalid direction: {s}")),
        };
        Ok(direction)
    }
}

impl From<Direction> for (isize, isize) {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, direction: Direction) -> Self::Output {
        let (dx, dy) = direction.into();
        (
            (self.0 as isize + dx) as usize,
            (self.1 as isize + dy) as usize,
        )
    }
}

struct Warehouse {
    map: Vec<Vec<State>>,
    robot: (usize, usize),
}

impl FromStr for Warehouse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot = (0, 0);
        let map = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        let state = c.to_string().parse().unwrap();
                        if state == State::Robot {
                            robot = (row, col);
                        }
                        state
                    })
                    .collect()
            })
            .collect();

        Ok(Warehouse { map, robot })
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            for state in line {
                write!(f, "{state}")?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Robot: {:?}", self.robot)?;
        Ok(())
    }
}

impl Warehouse {
    fn move_robot(&mut self, direction: Direction) {
        if let Some(new_pos) = self.empty_before_wall(direction) {
            self.map[self.robot.0][self.robot.1] = State::Empty;
            self.robot = self.robot + direction;
            if self.map[self.robot.0][self.robot.1] == State::Box {
                self.map[new_pos.0][new_pos.1] = State::Box;
            }
            self.map[self.robot.0][self.robot.1] = State::Robot;
        }
    }

    fn empty_before_wall(&self, direction: Direction) -> Option<(usize, usize)> {
        let mut new_pos = self.robot + direction;

        while self.map[new_pos.0][new_pos.1] != State::Empty
            && self.map[new_pos.0][new_pos.1] != State::Wall
        {
            new_pos = new_pos + direction;
        }

        if self.map[new_pos.0][new_pos.1] == State::Empty {
            Some(new_pos)
        } else {
            None
        }
    }

    fn apply_directions(&mut self, direction: &[Direction]) {
        for d in direction {
            self.move_robot(*d);
        }
    }

    fn gps_top_left(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, state)| **state == State::Box)
                    .map(|(col, _)| row * 100 + col)
                    .sum::<usize>()
            })
            .sum()
    }

    fn widen(&mut self) {
        let mut new_map = vec![vec![State::Wall; self.map[0].len() * 2]; self.map.len()];

        for (row, line) in self.map.iter().enumerate() {
            for (col, state) in line.iter().enumerate() {
                new_map[row][col * 2] = *state;
                new_map[row][col * 2 + 1] = match state {
                    State::Robot => State::Empty,
                    s => *s,
                };
            }
        }

        self.map = new_map;
    }
}

fn parse_input(input: &str) -> (Warehouse, Vec<Direction>) {
    let (map_part, directions_part) = input.split_once("\n\n").unwrap();

    let warehouse = map_part.parse().unwrap();

    let directions = directions_part
        .chars()
        .flat_map(|c| c.to_string().parse())
        .collect();

    (warehouse, directions)
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (mut warehouse, directions) = parse_input(input);
        warehouse.apply_directions(&directions);
        Some(warehouse.gps_top_left())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (mut warehouse, directions) = parse_input(input);
        warehouse.widen();
        warehouse.apply_directions(&directions);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 15;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(10_092));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1_514_353));
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

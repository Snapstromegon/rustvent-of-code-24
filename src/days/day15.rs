#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Add,
    str::FromStr,
};

use crate::solution::{Solution, SolvedValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Robot,
    Box,
    Box2,
    Wall,
    Empty,
}

impl State {
    fn to_string(self, widened: bool) -> char {
        match self {
            State::Robot => '@',
            State::Box => {
                if widened {
                    '['
                } else {
                    'O'
                }
            }
            State::Box2 => ']',
            State::Wall => '#',
            State::Empty => '.',
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    blocks: HashSet<(usize, usize)>,
    walls: HashSet<(usize, usize)>,
    robot: (usize, usize),
    size: (usize, usize),
    widened: bool,
}

impl FromStr for Warehouse {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut robot = (0, 0);
        let mut blocks = HashSet::new();
        let mut walls = HashSet::new();

        let mut size = (0, 0);

        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                size = (row, col);
                match c.to_string().parse()? {
                    State::Robot => {
                        robot = (row, col);
                    }
                    State::Box => {
                        blocks.insert((row, col));
                    }
                    State::Wall => {
                        walls.insert((row, col));
                    }
                    State::Empty => (),
                    State::Box2 => return Err(format!("Invalid character: {c}")),
                };
            }
        }

        Ok(Warehouse {
            blocks,
            walls,
            robot,
            size,
            widened: false,
        })
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                write!(f, "{}", self.get_state((row, col)).to_string(self.widened))?;
            }
            writeln!(f)?;
        }

        write!(f, "Robot: {:?}", self.robot)
    }
}

impl Warehouse {
    fn get_state(&self, pos: (usize, usize)) -> State {
        if self.widened && pos.1 > 0 {
            if self.blocks.contains(&(pos.0, pos.1 - 1)) {
                return State::Box2;
            } else if self.walls.contains(&(pos.0, pos.1 - 1)) {
                return State::Wall;
            }
        }
        if self.robot == pos {
            State::Robot
        } else if self.blocks.contains(&pos) {
            State::Box
        } else if self.walls.contains(&pos) {
            State::Wall
        } else {
            State::Empty
        }
    }

    fn move_robot(&mut self, direction: Direction) {
        if let Some(changes) = self.can_move_in_dir(self.robot, direction) {
            let mut new_blocks = HashSet::new();
            for (before, after) in changes {
                if self.blocks.contains(&before) {
                    self.blocks.remove(&before);
                    new_blocks.insert(after);
                }
            }
            self.blocks.extend(new_blocks);
            self.robot = self.robot + direction;
        }
    }

    fn can_move_in_dir(
        &self,
        start_pos: (usize, usize),
        direction: Direction,
    ) -> Option<HashMap<(usize, usize), (usize, usize)>> {
        let new_pos = start_pos + direction;
        if self.get_state(new_pos) == State::Wall {
            return None;
        }

        let mut changes = HashMap::new();
        changes.insert(start_pos, new_pos);
        if self.get_state(new_pos) == State::Empty {
            return Some(changes);
        }
        match (self.widened, direction) {
            (_, Direction::Left | Direction::Right) | (false, _) => {
                if let Some(other_changes) = self.can_move_in_dir(new_pos, direction) {
                    changes.extend(other_changes);
                } else {
                    return None;
                }
            }
            (true, Direction::Up | Direction::Down) => {
                let (left_changes, right_changes) = match self.get_state(new_pos) {
                    State::Box2 => (
                        self.can_move_in_dir((new_pos.0, new_pos.1 - 1), direction),
                        self.can_move_in_dir(new_pos, direction),
                    ),
                    State::Box => (
                        self.can_move_in_dir(new_pos, direction),
                        self.can_move_in_dir((new_pos.0, new_pos.1 + 1), direction),
                    ),
                    s => unreachable!("Invalid state: {:?}", s),
                };
                if let (Some(left_changes), Some(right_changes)) = (left_changes, right_changes) {
                    changes.extend(left_changes);
                    changes.extend(right_changes);
                } else {
                    return None;
                }
            }
        }

        Some(changes)
    }

    fn apply_directions(&mut self, direction: &[Direction]) {
        for d in direction {
            self.move_robot(*d);
        }
    }

    fn gps_sum(&self) -> usize {
        self.blocks.iter().map(|(row, col)| row * 100 + col).sum()
    }

    fn widen(&mut self) {
        self.widened = true;
        let wider = |(a, b): &(usize, usize)| (*a, b * 2);
        self.blocks = self.blocks.iter().map(wider).collect();
        self.walls = self.walls.iter().map(wider).collect();
        self.robot = wider(&self.robot);
        self.size = wider(&self.size);
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
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let (mut warehouse, directions) = parse_input(input);
        warehouse.apply_directions(&directions);
        Some(warehouse.gps_sum().into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let (mut warehouse, directions) = parse_input(input);
        warehouse.widen();
        warehouse.apply_directions(&directions);
        Some(warehouse.gps_sum().into())
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
        assert_eq!(Day.part1(&input), Some(10_092.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1_514_353.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(9_021.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1_533_076.into()));
    }
}

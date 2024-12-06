use rayon::prelude::*;
use std::str::FromStr;

use crate::solution::Solution;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridCell {
    Open,
    Obstacle,
    Visited,
}

#[derive(Debug, Clone)]
struct Field {
    grid: Vec<Vec<GridCell>>,
    guard_pos: Position,
    guard_dir: Direction,
    visited: Vec<Position>,
    visited_with_dir: Vec<Vec<Option<Direction>>>,
    rows: usize,
    cols: usize,
}

impl Field {
    fn next_guard_pos(&self) -> Position {
        self.guard_pos.add(self.guard_dir.vector())
    }

    fn is_step_possible(&self) -> bool {
        let next_pos = self.next_guard_pos();
        if (0..self.grid.len() as isize).contains(&next_pos.row)
            && (0..self.grid[0].len() as isize).contains(&next_pos.col)
        {
            self.grid[next_pos.row as usize][next_pos.col as usize] != GridCell::Obstacle
        } else {
            true
        }
    }

    fn guard_step(&mut self) {
        while !self.is_step_possible() {
            self.guard_dir = self.guard_dir.turn_right();
        }
        if self.grid[self.guard_pos.row as usize][self.guard_pos.col as usize] == GridCell::Open {
            self.visited.push(self.guard_pos);
        }
        self.grid[self.guard_pos.row as usize][self.guard_pos.col as usize] = GridCell::Visited;
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

    fn is_looping(&mut self) -> bool {
        let mut guard_state = (self.guard_pos, self.guard_dir);
        while self.is_guard_inside()
            && self.visited_with_dir[guard_state.0.row as usize][guard_state.0.col as usize]
                != Some(guard_state.1)
        {
            self.guard_step();
            self.visited_with_dir[guard_state.0.row as usize][guard_state.0.col as usize] =
                Some(guard_state.1);
            guard_state = (self.guard_pos, self.guard_dir);
        }

        self.is_guard_inside()
    }
}

impl FromStr for Field {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maybe_guard_pos = None;
        let mut rows = 0;
        let mut cols = 0;
        let mut grid = Vec::with_capacity(200);
        let mut visited_with_dir = Vec::with_capacity(200);

        for (row, line) in s.lines().enumerate() {
            rows = row;
            let mut grid_line = Vec::with_capacity(200);
            let mut visited_with_dir_line = Vec::with_capacity(200);
            for (col, char) in line.chars().enumerate() {
                cols = col;
                grid_line.push(match char {
                    '^' => {
                        maybe_guard_pos = Some(Position {
                            row: row as isize,
                            col: col as isize,
                        });
                        GridCell::Open
                    }
                    '#' => GridCell::Obstacle,
                    _ => GridCell::Open,
                });
                visited_with_dir_line.push(None);
            }
            grid.push(grid_line);
            visited_with_dir.push(visited_with_dir_line);
        }

        Ok(Field {
            grid,
            guard_pos: maybe_guard_pos.ok_or("No Guard found")?,
            guard_dir: Direction::Up,
            visited: Vec::new(),
            visited_with_dir,
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

    fn part2(&self, input: &str) -> Option<usize> {
        let field: Field = input.parse().expect("Field not parsable");

        let mut cloned = field.clone();
        cloned.simulate_to_exit();

        let looping_count = cloned
            .visited
            .par_iter()
            .filter(|Position { row, col }| {
                let mut field_clone = field.clone();
                field_clone.grid[*row as usize][*col as usize] = GridCell::Obstacle;
                field_clone.is_looping()
            })
            .count();

        Some(looping_count)
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
        assert_eq!(Day.part2(&input), Some(6));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1304));
    }
}

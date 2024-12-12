use std::{collections::HashSet, usize};

use crate::solution::Solution;

fn parse_input(input: &str) -> Vec<Vec<Option<char>>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Some(c)).collect())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default)]
struct Region {
    squares: HashSet<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.squares.len()
    }

    fn square_edges(&self, row: usize, col: usize) -> Vec<Direction> {
        let mut res = vec![];
        if row == 0 || !self.squares.contains(&(row - 1, col)) {
            res.push(Direction::Up);
        }
        if col == 0 || !self.squares.contains(&(row, col - 1)) {
            res.push(Direction::Left);
        }
        if !self.squares.contains(&(row + 1, col)) {
            res.push(Direction::Down);
        }
        if !self.squares.contains(&(row, col + 1)) {
            res.push(Direction::Right);
        }
        res
    }

    fn perimeter(&self) -> usize {
        let mut perim = 0;
        for (row, col) in &self.squares {
            perim += self.square_edges(*row, *col).len()
        }
        perim
    }

    fn bounds(&self) -> ((usize, usize), (usize, usize)) {
        let mut min_row = usize::MAX;
        let mut max_row = 0;
        let mut min_col = usize::MAX;
        let mut max_col = 0;

        for (row, col) in &self.squares {
            min_row = min_row.min(*row);
            max_row = max_row.max(*row);
            min_col = min_col.min(*col);
            max_col = max_col.max(*col);
        }

        ((min_row, max_row), (min_col, max_col))
    }

    fn sides(&self) -> usize {
        let bounds = self.bounds();
        let mut res = 0;
        for row in bounds.0 .0..=bounds.0 .1 {
            let mut last_up = usize::MAX - 1;
            let mut last_down = usize::MAX - 1;
            for col in bounds.1 .0..=bounds.1 .1 {
                if self.squares.contains(&(row, col)) {
                    let edges = self.square_edges(row, col);
                    if edges.contains(&Direction::Up) {
                        if col != last_up + 1 {
                            res += 1;
                        }
                        last_up = col;
                    }
                    if edges.contains(&Direction::Down) {
                        if col != last_down + 1 {
                            res += 1;
                        }
                        last_down = col;
                    }
                }
            }
        }

        for col in bounds.1 .0..=bounds.1 .1 {
            let mut last_left = usize::MAX - 1;
            let mut last_right = usize::MAX - 1;
            for row in bounds.0 .0..=bounds.0 .1 {
                if self.squares.contains(&(row, col)) {
                    let edges = self.square_edges(row, col);
                    if edges.contains(&Direction::Left) {
                        if row != last_left + 1 {
                            res += 1;
                        }
                        last_left = row;
                    }
                    if edges.contains(&Direction::Right) {
                        if row != last_right + 1 {
                            res += 1;
                        }
                        last_right = row;
                    }
                }
            }
        }

        res
    }
}

fn flood(map: &Vec<Vec<Option<char>>>, start_row: usize, start_col: usize) -> Region {
    let mut region = Region::default();
    let char = map[start_row][start_col].unwrap();
    let mut candidates = vec![(start_row, start_col)];
    while let Some(candidate) = candidates.pop() {
        if map[candidate.0][candidate.1] == Some(char) && !region.squares.contains(&candidate) {
            region.squares.insert(candidate);
            if candidate.0 > 0 {
                candidates.push((candidate.0 - 1, candidate.1));
            }
            if candidate.1 > 0 {
                candidates.push((candidate.0, candidate.1 - 1));
            }
            if candidate.0 < map.len() - 1 {
                candidates.push((candidate.0 + 1, candidate.1));
            }
            if candidate.1 < map[0].len() - 1 {
                candidates.push((candidate.0, candidate.1 + 1));
            }
        }
    }
    region
}

fn part1(mut map: Vec<Vec<Option<char>>>) -> usize {
    let mut total_price = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col].is_some() {
                let region = flood(&map, row, col);
                total_price += region.area() * region.perimeter();
                for (s_row, s_col) in region.squares {
                    map[s_row][s_col] = None;
                }
            }
        }
    }
    total_price
}

fn part2(mut map: Vec<Vec<Option<char>>>) -> usize {
    let mut total_price = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col].is_some() {
                let region = flood(&map, row, col);
                total_price += region.area() * region.sides();
                for (s_row, s_col) in region.squares {
                    map[s_row][s_col] = None;
                }
            }
        }
    }
    total_price
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let map = parse_input(input);
        Some(part1(map))
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let map = parse_input(input);
        Some(part2(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 12;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(140));
    }

    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1456082));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(80));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(872382));
    }
}

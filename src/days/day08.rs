use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    str::FromStr,
};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    col: isize,
    row: isize,
}

impl Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, other: (isize, isize)) -> Self {
        Position {
            col: self.col + other.1,
            row: self.row + other.0,
        }
    }
}

impl Sub<(isize, isize)> for Position {
    type Output = Self;

    fn sub(self, other: (isize, isize)) -> Self {
        Position {
            col: self.col - other.1,
            row: self.row - other.0,
        }
    }
}

#[derive(Debug)]
pub struct AntennaGrid {
    antennas: HashMap<char, Vec<Position>>,
    width: usize,
    height: usize,
}

impl AntennaGrid {
    fn get_antinodes(&self, resonants: bool) -> HashSet<Position> {
        let mut antinodes = HashSet::new();
        for positions in self.antennas.values() {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let self_pos = positions[i];
                    let other_pos = positions[j];

                    for antinode in self.get_pos_antinodes(self_pos, other_pos, resonants) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
        antinodes
    }

    fn pos_in_bounds(&self, pos: Position) -> bool {
        (0..self.width as isize).contains(&pos.col) && (0..self.height as isize).contains(&pos.row)
    }

    fn get_pos_antinodes(&self, a: Position, b: Position, resonants: bool) -> Vec<Position> {
        let mut antinodes = Vec::new();
        if resonants {
            antinodes.push(a);
            antinodes.push(b);
        }
        let vector = (b.row - a.row, b.col - a.col);
        let mut na = a - vector;
        if self.pos_in_bounds(na) {
            antinodes.push(na);
        }
        let mut nb = b + vector;
        if self.pos_in_bounds(nb) {
            antinodes.push(nb);
        }
        if resonants {
            while self.pos_in_bounds(na) {
                antinodes.push(na);
                na = na - vector;
            }
            while self.pos_in_bounds(nb) {
                antinodes.push(nb);
                nb = nb + vector;
            }
        }
        antinodes
    }
}

impl FromStr for AntennaGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas = HashMap::new();
        let lines = s.lines();
        for (row, line) in lines.clone().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != '.' {
                    antennas.entry(c).or_insert_with(Vec::new).push(Position {
                        col: col as isize,
                        row: row as isize,
                    });
                }
            }
        }
        Ok(AntennaGrid {
            antennas,
            height: lines.count(),
            width: s.lines().next().unwrap().len(),
        })
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let grid: AntennaGrid = input.parse().unwrap();
        Some(grid.get_antinodes(false).len())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let grid: AntennaGrid = input.parse().unwrap();
        Some(grid.get_antinodes(true).len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 8;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(14));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(269));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(34));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(949));
    }
}

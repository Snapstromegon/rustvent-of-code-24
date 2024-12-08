use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solution::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    col: usize,
    row: usize,
}

#[derive(Debug)]
pub struct AntennaGrid {
    antennas: HashMap<char, Vec<Position>>,
    width: usize,
    height: usize,
}

impl AntennaGrid {
    fn get_antinodes(&self) -> HashSet<Position> {
        let mut antinodes = HashSet::new();
        for (_, positions) in &self.antennas {
            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let self_pos = positions[i];
                    let other_pos = positions[j];

                    for antinode in self.get_pos_antinodes(self_pos, other_pos) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
        antinodes
    }

    fn pos_in_bounds(&self, row: isize, col: isize) -> bool {
        (0..self.width as isize).contains(&col) && (0..self.height as isize).contains(&row)
    }

    fn get_pos_antinodes(&self, a: Position, b: Position) -> Vec<Position> {
        let mut antinodes = Vec::new();
        let dx = b.col as isize - a.col as isize;
        let dy = b.row as isize - a.row as isize;
        let na = (a.row as isize - dy, a.col as isize - dx);
        if self.pos_in_bounds(na.0, na.1) {
            let na = Position {
                col: na.1 as usize,
                row: na.0 as usize,
            };
            antinodes.push(na);
        }
        let nb = (b.row as isize + dy, b.col as isize + dx);
        if self.pos_in_bounds(nb.0, nb.1) {
            let nb = Position {
                col: nb.1 as usize,
                row: nb.0 as usize,
            };
            antinodes.push(nb);
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
                    antennas
                        .entry(c)
                        .or_insert_with(Vec::new)
                        .push(Position { col, row });
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
        Some(grid.get_antinodes().len())
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
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
}

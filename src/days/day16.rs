#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::{self, Display, Formatter},
    ops::Add,
    str::FromStr,
};

use crate::solution::{Solution, SolvedValue};

type Position = (usize, usize);
type DirPos = (Position, Direction);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        let d = match rhs {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        };
        (
            (self.0 as isize + d.0) as usize,
            (self.1 as isize + d.1) as usize,
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DijkstraEntry {
    pos: Position,
    dir: Direction,
    cost: usize,
}

impl PartialOrd for DijkstraEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DijkstraEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

struct Maze {
    walls: Vec<Vec<bool>>,
    start: Position,
    end: Position,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => row.push(true),
                    '.' => row.push(false),
                    'S' => {
                        row.push(false);
                        start = (y, x);
                    }
                    'E' => {
                        row.push(false);
                        end = (y, x);
                    }
                    _ => panic!("Invalid character in maze"),
                }
            }
            walls.push(row);
        }
        Ok(Maze { walls, start, end })
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (y, row) in self.walls.iter().enumerate() {
            for (x, &wall) in row.iter().enumerate() {
                if (x, y) == self.start {
                    write!(f, "S")?;
                } else if (x, y) == self.end {
                    write!(f, "E")?;
                } else if wall {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Maze {
    fn dijkstra_to_end(
        &self,
    ) -> HashMap<DirPos, (usize, Vec<DirPos>)> {
        let mut costs: HashMap<
            DirPos,
            (usize, Vec<DirPos>),
        > = HashMap::new();

        let mut candidates = BinaryHeap::new();
        candidates.push(DijkstraEntry {
            pos: self.start,
            dir: Direction::East,
            cost: 0,
        });

        while let Some(entry) = candidates.pop() {
            if entry.pos == self.end {
                continue;
            }
            let (base_cost, _) = *costs
                .get(&(entry.pos, entry.dir))
                .unwrap_or(&(0usize, vec![]));
            let all_dirs = [
                (entry.dir, 1),
                (entry.dir.turn_left(), 1001),
                (entry.dir.turn_right(), 1001),
            ];
            let valid_dirs = all_dirs.iter().filter(|&&(dir, _)| {
                let next_pos = entry.pos + dir;
                !self.walls[next_pos.0][next_pos.1]
            });
            for (dir, cost) in valid_dirs {
                let next_pos = entry.pos + *dir;
                let next_cost = base_cost + cost;
                if let Some(&(prev_cost, _)) = costs.get(&(next_pos, *dir)) {
                    match next_cost.cmp(&prev_cost) {
                        std::cmp::Ordering::Less => {
                            costs.insert(
                                (next_pos, *dir),
                                (next_cost, vec![(entry.pos, entry.dir)]),
                            );
                            candidates.push(DijkstraEntry {
                                pos: next_pos,
                                dir: *dir,
                                cost: next_cost,
                            });
                        }
                        std::cmp::Ordering::Equal => {
                            costs
                                .get_mut(&(next_pos, *dir))
                                .unwrap()
                                .1
                                .push((entry.pos, entry.dir));
                        }
                        std::cmp::Ordering::Greater => {}
                    }
                } else {
                    costs.insert((next_pos, *dir), (next_cost, vec![(entry.pos, entry.dir)]));
                    candidates.push(DijkstraEntry {
                        pos: next_pos,
                        dir: *dir,
                        cost: next_cost,
                    });
                }
            }
        }

        costs
    }

    fn min_cost(&self) -> usize {
        let costs = self.dijkstra_to_end();
        [
            costs.get(&(self.end, Direction::North)),
            costs.get(&(self.end, Direction::East)),
            costs.get(&(self.end, Direction::South)),
            costs.get(&(self.end, Direction::West)),
        ]
        .iter()
        .filter_map(|x| x.map(|x| x.0))
        .min()
        .unwrap()
    }

    fn pos_on_best_paths(&self) -> usize {
        let min_cost = self.min_cost();
        let costs = self.dijkstra_to_end();
        let mut visited = HashSet::new();
        let mut candidates: Vec<DirPos> = vec![
            (self.end, Direction::North),
            (self.end, Direction::East),
            (self.end, Direction::South),
            (self.end, Direction::West),
        ]
        .into_iter()
        .filter(|end| {
            if let Some(x) = costs.get(end) {
                x.0 == min_cost
            } else {
                false
            }
        })
        .collect();
        while let Some(pos) = candidates.pop() {
            if !visited.contains(&pos) {
                visited.insert(pos);
                if let Some((_, prev_pos)) = costs.get(&pos) {
                    for &prev_pos in prev_pos {
                        candidates.push(prev_pos);
                    }
                }
            }
        }
        visited
            .iter()
            .map(|(pos, _)| *pos)
            .collect::<HashSet<Position>>()
            .len()
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let maze: Maze = input.parse().unwrap();
        let cost = maze.min_cost();
        Some(cost.into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let maze = input.parse::<Maze>().unwrap();
        let count = maze.pos_on_best_paths();
        Some(count.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 16;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(7_036.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(95_444.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(45.into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(513.into()));
    }

    #[test]
    fn direction_adds() {
        assert_eq!((5, 5) + Direction::North, (4, 5));
        assert_eq!((5, 5) + Direction::East, (5, 6));
        assert_eq!((5, 5) + Direction::South, (6, 5));
        assert_eq!((5, 5) + Direction::West, (5, 4));
    }
}

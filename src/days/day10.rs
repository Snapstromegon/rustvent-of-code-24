use std::{collections::HashSet, str::FromStr};

use crate::solution::Solution;

struct TopMap {
    map: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl FromStr for TopMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect()
            })
            .collect();
        Ok(TopMap {
            width: map[0].len(),
            height: map.len(),
            map,
        })
    }
}

impl TopMap {
    fn get_move_options(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut options = vec![];
        if x > 0 {
            options.push((x - 1, y));
        }
        if x < self.width - 1 {
            options.push((x + 1, y));
        }
        if y > 0 {
            options.push((x, y - 1));
        }
        if y < self.height - 1 {
            options.push((x, y + 1));
        }
        options
    }

    fn search_paths(&self, start_x: usize, start_y: usize, height: u8) -> HashSet<(usize, usize)> {
        let mut hs = HashSet::new();
        for (x, y) in self.get_move_options(start_x, start_y) {
            if self.map[y][x] == height {
                if height == 9 {
                    hs.insert((x, y));
                } else {
                    hs.extend(self.search_paths(x, y, height + 1));
                }
            }
        }
        hs
    }

    fn get_scores_sum(&self) -> usize {
        let mut scores_sum = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    let found_nines = self.search_paths(x, y, 1);
                    scores_sum += found_nines.len();
                }
            }
        }

        scores_sum
    }

    fn count_paths_to_nine(&self, start_x: usize, start_y: usize, height: u8) -> usize {
        let mut result = 0;
        for (x, y) in self.get_move_options(start_x, start_y) {
            if self.map[y][x] == height {
                if height == 9 {
                    result += 1;
                } else {
                    result += self.count_paths_to_nine(x, y, height + 1);
                }
            }
        }

        result
    }

    fn get_paths_sum(&self) -> usize {
        let mut scores_sum = 0;
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    scores_sum += self.count_paths_to_nine(x, y, 1);
                }
            }
        }
        scores_sum
    }
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let top_map: TopMap = input.parse().unwrap();
        Some(top_map.get_scores_sum())
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let top_map: TopMap = input.parse().unwrap();
        Some(top_map.get_paths_sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 10;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(36));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(811));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(81));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1794));
    }
}

use std::{collections::HashSet, str::FromStr};

use crate::solution::Solution;

fn parse_input(input: &str) -> ((isize, isize), Vec<Robot>) {
    let robots: Vec<Robot> = input.lines().map(|x| x.parse().unwrap()).collect();
    let max_x = robots.iter().map(|x| x.x).max().unwrap();
    let max_y = robots.iter().map(|x| x.y).max().unwrap();
    (
        if max_x > 11 || max_y > 7 {
            (101, 103)
        } else {
            (11, 7)
        },
        robots,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Robot {
    fn step(&mut self, count: isize, (width, height): (isize, isize)) {
        self.x = (self.x + self.dx * count + width * count) % width;
        self.y = (self.y + self.dy * count + height * count) % height;
    }

    fn quadrant(&self, (width, height): (isize, isize)) -> usize {
        if self.x == width / 2 || self.y == height / 2 {
            0
        } else if self.x > width / 2 && self.y > height / 2 {
            1
        } else if self.x < width / 2 && self.y > height / 2 {
            2
        } else if self.x < width / 2 && self.y < height / 2 {
            3
        } else {
            4
        }
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number_parts = s.split_whitespace().map(|x| x.split("=").nth(1).unwrap());
        let mut pos = number_parts
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap());
        let mut vec = number_parts
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse().unwrap());
        Ok(Self {
            x: pos.next().unwrap(),
            y: pos.next().unwrap(),
            dx: vec.next().unwrap(),
            dy: vec.next().unwrap(),
        })
    }
}

fn max_robots_block(robots: &[Robot], (width, height): (isize, isize)) -> usize {
    let mut robots_set = HashSet::new();
    for robot in robots {
        robots_set.insert((robot.x, robot.y));
    }
    let mut max_block_size = 0;
    for robot in robots {
        let mut visited = HashSet::new();
        let mut candidates = vec![];
        candidates.push((robot.x, robot.y));
        while !candidates.is_empty() {
            let (x, y) = candidates.pop().unwrap();
            if !visited.contains(&(x, y)) && robots_set.contains(&(x, y)) {
                visited.insert((x, y));
                if x > 0 {
                    candidates.push((x - 1, y));
                }
                if x < width - 1 {
                    candidates.push((x + 1, y));
                }
                if y > 0 {
                    candidates.push((x, y - 1));
                }
                if y < height - 1 {
                    candidates.push((x, y + 1));
                }
            }
        }
        max_block_size = max_block_size.max(visited.len());
    }
    max_block_size
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (size, mut robots) = parse_input(input);
        for robot in &mut robots {
            robot.step(100, size);
        }
        let res = robots
            .iter()
            .fold(vec![0; 5], |mut acc, robot| {
                acc[robot.quadrant(size)] += 1;
                acc
            })
            .iter()
            .skip(1)
            .fold(1, |acc, x| acc * x);
        Some(res)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (size, mut robots) = parse_input(input);
        for i in 1..10_000 {
            for robot in &mut robots {
                robot.step(1, size);
            }
            if max_robots_block(&robots, size)>100 {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 14;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(12));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(211773366));
    }

    #[test]
    #[ignore = "not possible on example input"]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), None);
    }
    #[test]
    #[ignore = "takes too long"]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(7344));
    }
}

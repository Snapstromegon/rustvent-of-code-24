use std::fmt::Display;

pub trait Solution {
    fn part1(&self, _input: &str) -> Option<usize> {
        None
    }
    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }

    fn run(&self, input: &str, part: Part) -> Option<usize> {
        match part {
            Part::One => self.part1(input),
            Part::Two => self.part2(input),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

impl From<Part> for u8 {
    fn from(part: Part) -> u8 {
        match part {
            Part::One => 1,
            Part::Two => 2,
        }
    }
}

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolvedValue {
    Usize(usize),
    String(String),
}

impl From<usize> for SolvedValue {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl From<&str> for SolvedValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for SolvedValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl Display for SolvedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolvedValue::Usize(value) => write!(f, "{value: >40}"),
            SolvedValue::String(value) => write!(f, "{value: >40}"),
        }
    }
}

pub trait Solution {
    fn part1(&self, _input: &str) -> Option<SolvedValue> {
        None
    }
    fn part2(&self, _input: &str) -> Option<SolvedValue> {
        None
    }

    fn run(&self, input: &str, part: Part) -> Option<SolvedValue> {
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

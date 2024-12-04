use crate::solution::Solution;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ]
    }

    pub fn get_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::UpRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, 1),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, -1),
        }
    }
}

fn search_in_dir(
    grid: &[Vec<char>],
    start_x: usize,
    start_y: usize,
    dir: Direction,
    search: &str,
) -> bool {
    let (dx, dy) = dir.get_vector();
    let mut x = start_x as i32;
    let mut y = start_y as i32;
    for c in search.chars() {
        if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
            return false;
        }
        if grid[y as usize][x as usize] != c {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn check_x_mas(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    // Safety:
    // We know that x-1, x+1, y-1, y+1 are within bounds
    // Also grid[y][x] == 'A'

    let top_left = grid[y - 1][x - 1];
    let top_right = grid[y - 1][x + 1];
    let bottom_left = grid[y + 1][x - 1];
    let bottom_right = grid[y + 1][x + 1];

    ((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
        && ((top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M'))
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut res = 0;
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 'X' {
                    for dir in Direction::all() {
                        if search_in_dir(&grid, x, y, dir, "XMAS") {
                            res += 1;
                        }
                    }
                }
            }
        }
        Some(res)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let mut res = 0;
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        for y in 1..grid.len() - 1 {
            for x in 1..grid[0].len() - 1 {
                if grid[y][x] == 'A' && check_x_mas(&grid, x, y) {
                    res += 1;
                }
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 4;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(18));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(2599));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(9));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(1948));
    }
}

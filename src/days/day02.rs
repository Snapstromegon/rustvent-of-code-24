use crate::solution::Solution;

pub struct Day;

fn get_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn report_okay(report: &[i32]) -> Option<(usize, usize)> {
    let mut increasing_okay = true;
    let mut decreasing_okay = true;
    let mut up_fail = 0;
    let mut down_fail = 0;

    for (i, item) in report.iter().enumerate().skip(1) {
        let last = report[i - 1];
        if !(1..=3).contains(&(item - last)) {
            increasing_okay = false;
            up_fail = i;
        }
        if !(1..=3).contains(&(last - item)) {
            decreasing_okay = false;
            down_fail = i;
        }
    }

    if increasing_okay || decreasing_okay {
        None
    } else {
        Some((up_fail, down_fail))
    }
}

fn check_is_save_part2(report: &[i32], has_skipped: bool) -> bool {
    let okay = report_okay(report);
    if okay.is_none() {
        return true;
    }
    if has_skipped {
        return false;
    }
    let (up_fail, down_fail) = okay.unwrap();
    let mut report_clone_up = report.to_owned();
    report_clone_up.remove(up_fail);
    let mut report_clone_up_before = report.to_owned();
    report_clone_up_before.remove(up_fail - 1);
    let mut report_clone_down = report.to_owned();
    report_clone_down.remove(down_fail);
    let mut report_clone_down_before = report.to_owned();
    report_clone_down_before.remove(down_fail - 1);
    let mut report_clone_first = report.to_owned();
    report_clone_first.remove(0);

    check_is_save_part2(&report_clone_first, true)
        || check_is_save_part2(&report_clone_up_before, true)
        || check_is_save_part2(&report_clone_down_before, true)
        || check_is_save_part2(&report_clone_up, true)
        || check_is_save_part2(&report_clone_down, true)
}

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let reports = get_reports(input);
        Some(
            reports
                .iter()
                .filter(|report| report_okay(report).is_none())
                .count(),
        )
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let reports = get_reports(input);
        let okay_reports = reports
            .iter()
            .filter(|report| check_is_save_part2(report, false))
            .count();
        Some(okay_reports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 2;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(2));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(407));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(4));
    }

    #[test]
    fn test_part2_custom1() {
        let input = "12 7 9 7 5 3";
        assert_eq!(Day.part2(&input), Some(1));
    }

    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(459));
    }
}

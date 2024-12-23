use std::collections::{HashMap, HashSet};

use crate::solution::{Solution, SolvedValue};

type ConnectionMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> ConnectionMap {
    let mut res: HashMap<&str, Vec<&str>> = HashMap::new();
    for connection in input.lines() {
        let parties = connection.split_once('-').unwrap();
        res.entry(parties.0).or_default().push(parties.1);
        res.entry(parties.1).or_default().push(parties.0);
    }
    res
}

fn is_pairwise_connected(connections: &ConnectionMap, partners: &[&str]) -> bool {
    for i in 0..partners.len() {
        for j in i + 1..partners.len() {
            if !connections.get(partners[i]).unwrap().contains(&partners[j]) {
                return false;
            }
        }
    }
    true
}

fn build_groups_of_size<'a>(
    connections: &'a ConnectionMap<'a>,
    size: usize,
    start: &'a str,
) -> HashSet<Vec<&'a str>> {
    if size == 0 {
        return HashSet::new();
    }
    if size == 1 {
        return HashSet::from_iter([vec![start]]);
    }
    let mut groups: HashSet<Vec<&str>> = HashSet::new();
    for other in connections.get(start).unwrap() {
        let sub_groups = build_groups_of_size(connections, size - 1, other);
        for sub_group in sub_groups {
            let mut group = vec![start];
            group.extend(sub_group);
            if is_pairwise_connected(connections, &group) {
                group.sort_unstable();
                groups.insert(group);
            }
        }
    }
    groups
}

fn biggest_pairwise_group<'a>(connections: &'a ConnectionMap) -> Vec<&'a str> {
    let mut biggest_group: Option<Vec<&str>> = None;
    for (&party, partners) in connections {
        let mut group = vec![party];
        for &partner in partners {
            group.push(partner);
            if is_pairwise_connected(connections, &group)
                && group.len() > biggest_group.as_ref().map_or(0, Vec::len)
            {
                biggest_group = Some(group.clone());
            }
        }
    }
    biggest_group.unwrap().clone()
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<SolvedValue> {
        let connections = parse_input(input);
        let mut t_groups = HashSet::new();
        for start in connections.keys().filter(|k| k.starts_with('t')) {
            t_groups.extend(build_groups_of_size(&connections, 3, start));
        }
        Some(t_groups.len().into())
    }

    fn part2(&self, input: &str) -> Option<SolvedValue> {
        let connections = parse_input(input);
        let mut biggest_group = biggest_pairwise_group(&connections);
        biggest_group.sort_unstable();
        Some(biggest_group.join(",").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;
    use crate::utils::read_input;

    const DAY: usize = 23;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(7.into()));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1_218.into()));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some("co,de,ka,ta".into()));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some("ah,ap,ek,fj,fr,jt,ka,ln,me,mp,qa,ql,zg".into()));
    }
}

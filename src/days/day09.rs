use crate::solution::Solution;

fn parse_input(input: &str) -> (usize, Vec<usize>, Vec<usize>) {
    let mut chars = input.chars();

    let mut used = Vec::new();
    let mut available = Vec::new();

    let mut size = 0;

    while let Some(c) = chars.next() {
        let used_size = c.to_string().parse().unwrap();
        used.push(used_size);
        size += used_size;
        if let Some(c) = chars.next() {
            let available_size = c.to_string().parse().unwrap();
            available.push(available_size);
            size += available_size;
        }
    }

    (size, used, available)
}

fn build_storage(size: usize, used: &[usize], available: &[usize]) -> Vec<Option<usize>> {
    let mut storage = Vec::with_capacity(size);

    for (id, block_size) in used.iter().enumerate() {
        for _ in 0..*block_size {
            storage.push(Some(id));
        }
        if let Some(available) = available.get(id) {
            for _ in 0..*available {
                storage.push(None);
            }
        }
    }

    storage
}

fn defrag(storage: &mut Vec<Option<usize>>) -> Vec<usize> {
    let mut defragged = Vec::new();

    while storage.len() > 0 {
        while storage.len() > 0 && storage[0].is_some() {
            defragged.push(storage.remove(0).unwrap());
        }
        while storage.len() > 0 && storage.first().unwrap().is_none() {
            while storage.len() > 0 && storage.last().unwrap().is_none() {
                storage.pop();
            }
            if storage.len() > 0 {
                storage.remove(0);
                defragged.push(storage.pop().unwrap().unwrap());
            }
        }
    }

    defragged
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (size, used, available) = parse_input(input);
        let storage = build_storage(size, &used, &available);
        let defragged = defrag(&mut storage.clone());
        let result = defragged.iter().enumerate().map(|(i, id)| i * *id).sum();
        Some(result)
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

    const DAY: usize = 9;

    #[test]
    fn test_part1_example() {
        let input = read_input(DAY, true, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(1928));
    }
    #[test]
    fn test_part1_challenge() {
        let input = read_input(DAY, false, 1).unwrap();
        assert_eq!(Day.part1(&input), Some(6291146824486));
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

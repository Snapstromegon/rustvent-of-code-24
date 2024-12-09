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

fn defrag(storage: &mut Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut defragged = storage.clone();

    for i in 0..defragged.len() {
        if defragged[i] == None {
            if let Some(next_some) = (i..defragged.len())
                .rev()
                .find(|j| defragged.get(*j).unwrap_or(&None).is_some())
            {
                defragged.swap(i, next_some);
            }
        }
    }

    defragged
}

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Used(usize, usize),
    Free(usize),
}

fn merge_blocks(used: &[usize], available: &[usize]) -> Vec<Option<usize>> {
    let mut block_list = Vec::new();

    for i in 0..used.len() {
        block_list.push(Block::Used(i, used[i]));
        if i < available.len() {
            block_list.push(Block::Free(available[i]));
        }
    }

    'outer: for i in (0..used.len()).rev() {
        let block_index = block_list
            .iter()
            .position(|b| matches!(b, Block::Used(id, _) if *id==i))
            .unwrap();
        for j in 0..block_index {
            if let Block::Free(size) = block_list[j] {
                if let Block::Used(_, block_size) = block_list[block_index] {
                    if size > block_size {
                        block_list[j] = Block::Free(size - block_size);
                        let block = block_list.remove(block_index);
                        block_list.insert(block_index, Block::Free(block_size));
                        block_list.insert(j, block);
                        continue 'outer;
                    }
                    if size == block_size {
                        let block = block_list.remove(block_index);
                        block_list.insert(block_index, Block::Free(block_size));
                        block_list[j] = block;
                        continue 'outer;
                    }
                }
            }
        }
    }

    let mut result = vec![];
    for block in block_list {
        match block {
            Block::Free(size) => {
                for _ in 0..size {
                    result.push(None);
                }
            }
            Block::Used(id, size) => {
                for _ in 0..size {
                    result.push(Some(id));
                }
            }
        }
    }
    result
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (size, used, available) = parse_input(input);
        let storage = build_storage(size, &used, &available);
        let defragged = defrag(&mut storage.clone());
        let result = defragged
            .iter()
            .enumerate()
            .filter(|(_, block)| block.is_some())
            .map(|(i, id)| i * id.unwrap())
            .sum();
        Some(result)
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (_size, used, available) = parse_input(input);
        let merged = merge_blocks(&used, &available);
        let result = merged
            .iter()
            .enumerate()
            .filter(|(_, block)| block.is_some())
            .map(|(i, id)| i * id.unwrap())
            .sum();
        Some(result)
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
        assert_eq!(Day.part2(&input), Some(2858));
    }
    #[test]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(6307279963620));
    }
}

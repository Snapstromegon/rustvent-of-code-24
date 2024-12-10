use std::collections::VecDeque;

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

#[derive(Debug, PartialEq, Eq)]
enum Block {
    Used(usize, usize),
    Free(usize),
}

impl Block {
    fn size(&self) -> usize {
        match self {
            Self::Free(size) | Self::Used(_, size) => *size,
        }
    }

    fn free_size(&self) -> usize {
        match self {
            Self::Free(size) => *size,
            Self::Used(..) => 0,
        }
    }

    fn id(&self) -> Option<usize> {
        match self {
            Self::Free(_) => None,
            Self::Used(id, _) => Some(*id),
        }
    }
}

fn marge_chunked(used: &[usize], available: &[usize]) -> usize {
    let mut used_blocks = used.iter().copied().enumerate().collect::<VecDeque<_>>();
    let mut available = available.to_vec();
    let mut iter_avail = available.iter_mut();
    let mut layout = vec![];

    while !used_blocks.is_empty() {
        layout.push(used_blocks.pop_front().unwrap());
        if let Some(avail) = iter_avail.next() {
            while !used_blocks.is_empty() && *avail > 0 {
                let (id, next_used_size) = used_blocks.pop_back().unwrap();
                if next_used_size <= *avail {
                    *avail -= next_used_size;
                    layout.push((id, next_used_size));
                } else {
                    layout.push((id, *avail));
                    used_blocks.push_back((id, next_used_size - *avail));
                    *avail = 0;
                }
            }
        }
    }

    let mut result = 0;
    let mut pos = 0;
    for (id, size) in layout {
        let base = pos * size;
        let top_triangle = size * (size.max(1) - 1) / 2;
        result += id * (top_triangle + base);
        pos += size;
    }

    result
}

fn merge_blocks(used: &[usize], available: &[usize]) -> usize {
    let mut block_list = Vec::new();

    for i in 0..used.len() {
        block_list.push(Block::Used(i, used[i]));
        if i < available.len() {
            block_list.push(Block::Free(available[i]));
        }
    }

    for i in (0..used.len()).rev() {
        let block_index = block_list.iter().position(|b| b.id() == Some(i)).unwrap();
        let block_size = block_list[block_index].size();
        let candidate = (0..block_index)
            .map(|j| (j, &block_list[j]))
            .find(|(_, block)| block.free_size() >= block_size);
        if let Some((j, block)) = candidate {
            let free_size = block.free_size();
            match free_size.cmp(&block_size) {
                std::cmp::Ordering::Greater => {
                    block_list[j] = Block::Free(free_size - block_size);
                    block_list.push(Block::Free(block_size));
                    let block = block_list.swap_remove(block_index);
                    block_list.insert(j, block);
                }
                std::cmp::Ordering::Equal => {
                    block_list.swap(j, block_index);
                }
                std::cmp::Ordering::Less => {}
            }
        }
    }

    let mut result = 0;
    let mut pos = 0;
    for block in block_list {
        match block {
            Block::Free(size) => {
                pos += size;
            }
            Block::Used(id, size) => {
                let base = pos * size;
                let top_triangle = size * (size.max(1) - 1) / 2;
                result += id * (top_triangle + base);
                pos += size;
            }
        }
    }
    result
}

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let (_size, used, available) = parse_input(input);
        Some(marge_chunked(&used, &available))
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let (_size, used, available) = parse_input(input);
        let merged = merge_blocks(&used, &available);
        Some(merged)
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
        assert_eq!(Day.part1(&input), Some(6_291_146_824_486));
    }

    #[test]
    fn test_part2_example() {
        let input = read_input(DAY, true, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(2858));
    }
    #[test]
    #[ignore = "takes too long"]
    fn test_part2_challenge() {
        let input = read_input(DAY, false, 2).unwrap();
        assert_eq!(Day.part2(&input), Some(6_307_279_963_620));
    }
}

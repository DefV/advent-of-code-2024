use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone)]
enum Block {
    File(u32, u32),
    Free(u32),
}

struct Diskmap {
    map: Vec<Block>,
}

impl From<&str> for Diskmap {
    fn from(s: &str) -> Self {
        let map = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let length = c.to_digit(10).unwrap();
                if i % 2 == 0 {
                    Block::File(length, (i / 2) as u32)
                } else {
                    Block::Free(length)
                }
            })
            .collect();

        Self { map }
    }
}

impl Diskmap {
    fn checksum(blocks: &[u32]) -> u64 {
        blocks
            .iter()
            .enumerate()
            .fold(0 as u64, |acc, (i, v)| acc + i as u64 * *v as u64)
    }
    fn reformat(&self) -> Vec<u32> {
        let mut map = VecDeque::from(self.map.clone());
        let mut result = vec![];

        let mut remains: Vec<u32> = vec![];

        while let Some(block) = map.pop_front() {
            match block {
                Block::File(length, idx) => {
                    result.extend(vec![idx; length as usize]);
                }
                Block::Free(length) => {
                    for _ in 0..length {
                        if remains.len() > 0 {
                            result.push(remains.pop().unwrap());
                        } else {
                            while let Some(block) = map.pop_back() {
                                match block {
                                    Block::File(length, idx) => {
                                        for _ in 0..length {
                                            remains.push(idx);
                                        }
                                        break;
                                    }
                                    Block::Free(_) => {}
                                }
                            }
                            if let Some(r) = remains.pop() {
                                result.push(r);
                            }
                        }
                    }
                }
            }
        }

        for r in remains {
            result.push(r);
        }

        result
    }

    fn whole_block_reformat(&self) -> Vec<u32> {
        let mut moved: HashSet<u32> = HashSet::new();
        let mut map = VecDeque::from(self.map.clone());
        let mut result: VecDeque<Block> = VecDeque::new();

        while let Some(block) = map.pop_back() {
            match block {
                Block::File(length, idx) => {
                    if !moved.insert(idx) {
                        result.push_front(block);
                        continue;
                    }

                    let mut found_idx = 0;
                    let mut free_remain = 0;
                    for (i, lblock) in map.iter().enumerate() {
                        match lblock {
                            Block::Free(free_size) => {
                                if *free_size >= length {
                                    found_idx = i;
                                    free_remain = *free_size - length;
                                    map.push_back(Block::Free(length));
                                    break;
                                }
                            }
                            Block::File(_, _) => {}
                        }
                    }

                    if found_idx != 0 {
                        map[found_idx] = block;
                        if free_remain > 0 {
                            map.insert(found_idx + 1, Block::Free(free_remain));
                        }
                    } else {
                        result.push_front(block);
                    }
                }
                Block::Free(_) => {
                    result.push_front(block);
                }
            }
        }

        result.iter().flat_map(|b| match b {
            Block::File(size, idx) => vec![*idx; *size as usize],
            Block::Free(size) => vec![0; *size as usize],
        }).collect()
    }
}
fn main() {
    let input = aoc::input();
    let input = input.trim();

    let diskmap = Diskmap::from(input);
    let result = Diskmap::checksum(&diskmap.reformat());

    println!("Part 1 checksum: {}", result);

    let result = Diskmap::checksum(&diskmap.whole_block_reformat());
    println!("Part 2 checksum: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reformat() {
        let diskmap = Diskmap::from("12345");
        let result = diskmap.reformat();
        assert_eq!(result, vec![0, 2, 2, 1, 1, 1, 2, 2, 2]);
    }
}

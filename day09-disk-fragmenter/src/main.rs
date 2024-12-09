use std::collections::VecDeque;

#[derive(Debug,Clone)]
enum Block {
    File(u32, u32),
    Free(u32)
}

struct Diskmap {
    map: Vec<Block>
}

impl From<&str> for Diskmap {
    fn from(s: &str) -> Self {
        let map = s.chars().enumerate().map(|(i, c)| {
            let length = c.to_digit(10).unwrap();
            if i % 2 == 0 {
                Block::File(length, (i / 2) as u32)
            } else {
                Block::Free(length)
            }
        }).collect();

        Self { map }
    }
}

impl Diskmap {
    fn reformat(self) -> Vec<u32> {
        let mut map = VecDeque::from(self.map);
        let mut result = vec![];

        let mut remains: Vec<u32> = vec![];

        while let Some(block) = map.pop_front() {
            match block {
                Block::File(length, idx) => {
                    for _ in 0..length {
                        result.push(idx);
                    }
                },
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
                                    },
                                    Block::Free(_) => {
                                    }
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

}
fn main() {
    let input = aoc::input();
    let input =input.trim();

    let diskmap = Diskmap::from(input);
    let result = diskmap.reformat().iter().enumerate().fold(0 as u64, |acc, (i, v)| {
        acc + i as u64 * *v as u64
    });

    println!("Part 1 checksum: {}", result);
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

use std::collections::{HashMap, VecDeque};

struct Stones {
    stones: Vec<u64>,
    cache: HashMap<(u64, u32), u64>
}

impl From<&str> for Stones {
    fn from(input: &str) -> Self {
        let stones = input.split_whitespace().map(|s| s.parse().unwrap() ).collect();

        Self { stones, cache: HashMap::new() }
    }
}

impl Stones {
    fn blink(&mut self, n: u32, queue: Option<Vec<u64>>) -> u64 {
        let queue = queue.or(Some(self.stones.clone())).unwrap();

        queue.iter().map(|&stone| {
            if let Some(&count) = self.cache.get(&(stone, n)) {
                return count;
            }

            self.blink_stone(stone).iter().map(|&new_stone| {
                if n == 1 {
                    return 1;
                }

                let count = self.blink(n - 1, Some(vec![new_stone]));
                self.cache.insert((new_stone, n - 1), count);

                count
            }).sum()
        }).sum()
    }

    fn blink_stone(&mut self, stone: u64) -> Vec<u64> {
        if stone == 0 {
            return vec![1];
        }

        let num_digits = (stone as f64).log10().floor() as u32 + 1;
        if num_digits % 2 == 0 {
            let left_stone = stone / 10u64.pow(num_digits / 2);
            let right_stone = stone % 10u64.pow(num_digits / 2);

            return vec![left_stone, right_stone]
        }

        return vec![stone * 2024]
    }
}

fn main() {
    let input = aoc::input();
    let input = input.trim();
    let mut stones = Stones::from(input);

    println!("Step 1: {}", stones.blink(25, None));
    println!("Step 2: {}", stones.blink(75, None));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_1_blink() {
        let mut stones = Stones::from(EXAMPLE);
        assert_eq!(stones.blink(1, None), 3);
    }

    #[test]
    fn test_2_blink() {
        let mut stones = Stones::from(EXAMPLE);
        assert_eq!(stones.blink(2, None), 4);
    }

    #[test]
    fn test_25_blink() {
        let mut stones = Stones::from(EXAMPLE);

        assert_eq!(stones.blink(25, None), 55312);
    }
}

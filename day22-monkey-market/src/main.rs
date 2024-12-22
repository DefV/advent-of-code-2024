use std::collections::{HashMap, HashSet};
use itertools::Itertools;

// Let's start with the naive approach
struct Trader {
    secret: u64
}

impl From<&str> for Trader {
    fn from(input: &str) -> Self {
        let secret = input.parse().unwrap();
        Trader { secret }
    }
}

const BITMASK: u64 = 0b111111111111111111111111;
impl Trader {
    fn shuffle(secret: &mut u64) {
        *secret = (*secret << 6 ^ *secret) & BITMASK;
        *secret = (*secret >> 5 ^ *secret) & BITMASK;
        *secret = (*secret << 11 ^ *secret) & BITMASK;
    }

    fn regen_secret(&self, times: usize) -> u64 {
        let mut secret = self.secret;

        for _ in 0..times {
            Self::shuffle(&mut secret);
        }

        secret
    }

    fn price_fluctuations(&self, times: usize) -> Vec<(usize, isize)> {
        let mut secret = self.secret;
        let mut differences = Vec::new();

        let mut last: Option<usize> = None;

        for _ in 0..times {
            Self::shuffle(&mut secret);
            let current = (secret % 10) as usize;
            if let Some(last) = last {
                differences.push((current, current as isize - (last as isize)));
            }

            last = Some(current);
        }

        differences
    }
}

fn main() {
    let input = aoc::input();
    let result: u64 = input.trim().lines().map(Trader::from).map(|trader| {
        trader.regen_secret(2000)
    }).sum();

    println!("Part 1: {}", result);

    let mut sequence_hash: HashMap<[isize; 4], usize> = HashMap::new();

    input.trim().lines().map(Trader::from).for_each(|trader| {
        let mut seen_sequence: HashSet<[isize; 4]> = HashSet::new();

        for (first, second, third, last) in trader.price_fluctuations(2000).iter().tuple_windows() {
            let sequence = [first.1, second.1, third.1, last.1];
            if !seen_sequence.contains(&sequence) {
                seen_sequence.insert(sequence);
                sequence_hash.entry(sequence).and_modify(|count| *count += last.0).or_insert(last.0);
            }
        }
    });

    let result = sequence_hash.values().max().unwrap();
    println!("Part 2: {}", result);
}

use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
struct Puzzle<'a> {
    options: Vec<(&'a str, usize)>,
    towels: Vec<&'a str>,
}

impl<'a> From<&'a str> for Puzzle<'a> {
    fn from(input: &'a str) -> Self {
        let (options_data, towels_data) = input.split_once("\n\n").unwrap();

        Puzzle {
            options: options_data.split(", ").map(|option| (option, option.len())).collect(),
            towels: towels_data.lines().collect(),
        }
    }
}

impl<'a> Puzzle<'a> {
    fn makeable_options(&self) -> Vec<u64> {
        let mut makeable_cache: HashMap<&'a [u8], u64> = HashMap::new();
        self.towels
            .iter()
            .copied()
            .map(|towel| self.makeable_ways(towel.as_bytes(), &mut makeable_cache))
            .collect()
    }

    fn makeable_ways(&self, towel: &'a [u8], cache: &mut HashMap<&'a [u8], u64>) -> u64 {
        if let Some(&ways) = cache.get(towel) {
            return ways;
        }

        if towel.is_empty() {
            return 1;
        }

        let mut result = 0;

        for &(option, len) in &self.options {
            if towel.starts_with(option.as_bytes()) {
                let remaining = &towel[len..];
                result += self.makeable_ways(remaining, cache);
            }
        }

        cache.insert(towel, result);
        result
    }
}

fn main() {
    let start = Instant::now();
    let input = aoc::input();
    let puzzle: Puzzle = input.as_str().into();

    println!("Part 1: {}, {:?}", puzzle.makeable_options().iter().filter(|&&count| count > 0).count(), start.elapsed());
    println!("Part 2: {}, {:?}", puzzle.makeable_options().iter().sum::<u64>(), start.elapsed());
}

use std::collections::HashMap;

#[derive(Debug)]
struct Puzzle<'a> {
    options: Vec<&'a str>,
    towels: Vec<&'a str>,
}

impl<'a> From<&'a str> for Puzzle<'a> {
    fn from(input: &'a str) -> Self {
        let (options_data, towels_data) = input.split_once("\n\n").unwrap();

        Puzzle {
            options: options_data.split(", ").collect(),
            towels: towels_data.lines().collect(),
        }
    }
}

impl<'a> Puzzle<'a> {
    fn makeable_options(&self) -> Vec<u64> {
        let mut makeable_cache: HashMap<&'a str, u64> = HashMap::new();
        self.towels
            .iter()
            .copied()
            .map(|towel| self.makeable_ways(towel, &mut makeable_cache))
            .collect()
    }

    fn makeable_ways(&self, towel: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {
        if let Some(&ways) = cache.get(towel) {
            return ways;
        }

        if towel.is_empty() {
            return 1;
        }

        let result = self.options.iter().map(|option| {
            if towel.starts_with(option) {
                let remaining = &towel[option.len()..];

                self.makeable_ways(remaining, cache)
            } else {
                0
            }
        }).sum();

        cache.insert(towel, result);
        result
    }
}

fn main() {
    let input = aoc::input();
    let puzzle: Puzzle = input.as_str().into();

    println!("Part 1: {}", puzzle.makeable_options().iter().filter(|&&count| count > 0).count());
    println!("Part 2: {}", puzzle.makeable_options().iter().sum::<u64>());
}

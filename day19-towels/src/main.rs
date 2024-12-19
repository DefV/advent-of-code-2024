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
    fn makeable_towels(&self) -> Vec<&'a str> {
        let mut makeable_cache: HashMap<&'a str, bool> = HashMap::new();
        self.towels
            .iter()
            .copied()
            .filter(|towel| dbg!(self.is_makeable(towel, &mut makeable_cache)))
            .collect()
    }

    fn is_makeable(&self, towel: &'a str, cache: &mut HashMap<&'a str, bool>) -> bool {
        if let Some(&result) = cache.get(towel) {
            return result;
        }

        let result = self.options.iter().any(|option| {
            if towel.starts_with(option) {
                let remaining = &towel[option.len()..];
                if remaining.is_empty() || self.is_makeable(remaining, cache) {
                    return true;
                }
            }
            false
        });

        cache.insert(towel, result);
        result
    }
}

fn main() {
    let input = aoc::input();
    let puzzle: Puzzle = input.as_str().into();

    println!("Part 1: {}", puzzle.makeable_towels().len());
}

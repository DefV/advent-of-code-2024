use std::collections::HashMap;

type OrderingRule = (usize, usize);
type Update = Vec<usize>;

struct Puzzle {
    rules: Vec<OrderingRule>,
    updates: Vec<Update>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let (rule_input, update_input) = input.split_once("\n\n").unwrap();
        let rules = rule_input
            .lines()
            .filter_map(|line| {
                line.split_once("|")
                    .and_then(|(l, r)| Some((l.parse::<usize>().ok()?, r.parse::<usize>().ok()?)))
            })
            .collect();

        let updates = update_input
            .lines()
            .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
            .collect();

        Self { rules, updates }
    }
}

impl Puzzle {
    fn is_valid(&self, update: &Update) -> bool {
        let update_set: HashMap<&usize, usize> = update
            .iter()
            .enumerate()
            .map(|(idx, val)| (val, idx))
            .collect();
        let relevant_rules: Vec<&OrderingRule> = self
            .rules
            .iter()
            .filter(|(l, r)| update_set.contains_key(&l) && update_set.contains_key(&r))
            .collect();

        relevant_rules.iter().all(|(l, r)| {
            let l_idx = update_set[l];
            let r_idx = update_set[r];
            l_idx < r_idx
        })
    }

    fn valid_updates(&self) -> Vec<Update> {
        self.updates
            .iter()
            .filter(|update| self.is_valid(update))
            .cloned()
            .collect()
    }
}

fn main() {
    let input = aoc::input();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> u32 {
    Puzzle::from(input).valid_updates().iter()
        .map(|update| update[update.len() / 2] as u32)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_from_str() {
        let input = "0|1\n2|3\n\n1,2,3\n4,5,6";

        let puzzle = Puzzle::from(input);
        assert_eq!(puzzle.rules, vec![(0, 1), (2, 3)]);
        assert_eq!(puzzle.updates, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }
}
